//! The generic bounded row viewer over one introspected `public` table.
//!
//! This is the closest Studio comes to a SQL console, and it is deliberately not
//! one. The operator chooses a table name and a page, never a fragment of SQL.
//! The table name is validated against live introspection before it is used, the
//! identifiers Studio writes into the statement come from `pg_catalog` and are
//! re-checked and quoted, and every cursor value is a bound parameter cast to the
//! column's own introspected type. The statement is always a `SELECT` with a hard
//! `LIMIT`; nothing here can write.

use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;

use super::page::{self, Bound};
use super::schema;
use crate::StudioError;

/// The alias the projection gives the table, so `row_to_json` has a composite to
/// serialize and every reference in the statement is qualified.
const ROW_ALIAS: &str = "studio_row";

/// PostgreSQL's own identifier length limit.
const MAX_IDENTIFIER_LENGTH: usize = 63;

/// A generous bound on a `format_type` rendering such as
/// `character varying(64)` or `timestamp with time zone`.
const MAX_TYPE_LENGTH: usize = 128;

#[derive(Debug, Serialize)]
pub struct RowPage {
    pub table: String,
    /// The table's columns in storage order; every row object carries these keys.
    pub column: Vec<String>,
    /// The primary-key columns, in key order. Empty when the table has none.
    pub primary_key: Vec<String>,
    pub row: Vec<Value>,
    /// The last row's primary-key values as text, to continue the keyset.
    pub next_cursor: Option<Vec<String>>,
    pub truncated: bool,
}

/// One bounded page of rows from one introspected `public` table.
///
/// `table` must name a current application table: it is compared against
/// [`schema::application_table`], which reads `pg_class` live, and anything else
/// is [`StudioError::NotFound`]. A name is never escaped into safety — it is
/// either in the introspected list or rejected.
///
/// Ordering is the table's primary key, so paging is an index range and a keyset
/// with a row comparison — `(pk1, pk2) > ($1, $2)` — resumes exactly, whatever
/// the table's size. Cursor values arrive as text and are bound as text, then
/// cast in SQL to each key column's own `format_type`, so a `uuid` key compares
/// as a uuid and a `bigint` key as an integer rather than as a string.
///
/// A table without a primary key has no bounded stable order to key on. Studio
/// refuses it with [`StudioError::UnpageableTable`] instead of falling back to
/// physical row order.
pub async fn list_row(
    pool: &PgPool,
    table: &str,
    after: Option<&[String]>,
    bound: Bound,
) -> Result<RowPage, StudioError> {
    if !schema::application_table(pool)
        .await?
        .iter()
        .any(|known| known == table)
    {
        return Err(StudioError::NotFound);
    }
    let quoted_table = quote_identifier(table)?;
    let column = schema::table_column(pool, table).await?;
    let primary_key = schema::primary_key_column(pool, table).await?;
    if primary_key.is_empty() {
        return Err(StudioError::UnpageableTable);
    }
    let key_column = primary_key
        .iter()
        .map(|name| {
            let column = column
                .iter()
                .find(|column| &column.name == name)
                .ok_or(StudioError::NotFound)?;
            Ok((quote_identifier(name)?, cast_type(&column.data_type)?))
        })
        .collect::<Result<Vec<_>, StudioError>>()?;

    let cursor_value = key_column
        .iter()
        .map(|(name, _)| format!("{ROW_ALIAS}.{name}::text"))
        .collect::<Vec<_>>()
        .join(", ");
    let cursor_projection = format!("ARRAY[{cursor_value}]");
    let mut parameter = 0;
    let where_clause = match after {
        Some(after) if after.len() == key_column.len() => {
            let left = key_column
                .iter()
                .map(|(name, _)| format!("{ROW_ALIAS}.{name}"))
                .collect::<Vec<_>>()
                .join(", ");
            let right = key_column
                .iter()
                .map(|(_, data_type)| {
                    parameter += 1;
                    format!("${parameter}::{data_type}")
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("WHERE ({left}) > ({right})")
        }
        Some(_) => return Err(StudioError::NotFound),
        None => String::new(),
    };
    let order = key_column
        .iter()
        .map(|(name, _)| format!("{ROW_ALIAS}.{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let order_clause = format!("ORDER BY {order}");
    parameter += 1;
    let sql = format!(
        r#"
        SELECT row_to_json({ROW_ALIAS})::text AS document,
               {cursor_projection} AS cursor_value
        FROM public.{quoted_table} AS {ROW_ALIAS}
        {where_clause}
        {order_clause}
        LIMIT ${parameter}
        "#
    );

    let mut query = sqlx::query_as::<_, (String, Vec<String>)>(&sql);
    if let Some(after) = after {
        for value in after {
            query = query.bind(value.clone());
        }
    }
    let mut fetched = query.bind(bound.fetch()).fetch_all(pool).await?;
    let truncated = page::truncate(&mut fetched, bound.limit());
    let next_cursor = match (truncated, fetched.last()) {
        (true, Some((_, cursor))) => Some(cursor.clone()),
        _ => None,
    };
    let row = fetched
        .into_iter()
        .map(|(document, _)| {
            serde_json::from_str::<Value>(&document)
                .map_err(|error| StudioError::Database(sqlx::Error::Decode(Box::new(error))))
        })
        .collect::<Result<Vec<_>, StudioError>>()?;

    Ok(RowPage {
        table: table.to_owned(),
        column: column.into_iter().map(|column| column.name).collect(),
        primary_key,
        row,
        next_cursor,
        truncated,
    })
}

/// Quotes one introspected identifier for literal use in a Studio statement.
///
/// Every name reaching this function already came out of `pg_catalog`, but
/// Studio never escapes a string into safety: anything outside the ASCII
/// identifier alphabet — a quote, a space, a semicolon, a dash — is rejected
/// rather than quoted, so no interpolated identifier can ever close its own
/// quoting.
fn quote_identifier(name: &str) -> Result<String, StudioError> {
    let valid = !name.is_empty()
        && name.len() <= MAX_IDENTIFIER_LENGTH
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '$'));
    if !valid {
        return Err(StudioError::NotFound);
    }
    Ok(format!("\"{name}\""))
}

/// Validates one introspected `format_type` rendering before it becomes a cast.
///
/// The renderings PostgreSQL produces for current columns are names, spaces and
/// size modifiers — `uuid`, `bigint`, `timestamp with time zone`,
/// `character varying(64)`, `text[]`. Anything else is rejected instead of
/// interpolated.
fn cast_type(data_type: &str) -> Result<&str, StudioError> {
    let valid = !data_type.is_empty()
        && data_type.len() <= MAX_TYPE_LENGTH
        && data_type.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(character, ' ' | '_' | '(' | ')' | ',' | '[' | ']')
        });
    if !valid {
        return Err(StudioError::NotFound);
    }
    Ok(data_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_identifier_is_accepted_only_when_it_needs_no_escaping() {
        assert_eq!(quote_identifier("entity").unwrap(), "\"entity\"");
        assert_eq!(quote_identifier("user").unwrap(), "\"user\"");
        assert_eq!(
            quote_identifier("_sqlx_migrations").unwrap(),
            "\"_sqlx_migrations\""
        );
        for rejected in [
            "",
            "entity\"",
            "\"entity\"",
            "entity; DROP TABLE entity",
            "entity name",
            "entity-name",
            "entity--",
            &"a".repeat(MAX_IDENTIFIER_LENGTH + 1),
        ] {
            assert!(
                matches!(quote_identifier(rejected), Err(StudioError::NotFound)),
                "identifier must be rejected: {rejected}"
            );
        }
    }

    #[test]
    fn a_cast_type_is_accepted_only_when_it_is_a_rendered_postgresql_type() {
        for accepted in [
            "uuid",
            "bigint",
            "text",
            "bytea",
            "boolean",
            "timestamp with time zone",
            "character varying(64)",
            "numeric(10,2)",
            "text[]",
        ] {
            assert_eq!(cast_type(accepted).unwrap(), accepted);
        }
        for rejected in ["", "uuid; DROP TABLE entity", "uuid'", "uuid\"", "uuid--"] {
            assert!(
                matches!(cast_type(rejected), Err(StudioError::NotFound)),
                "cast type must be rejected: {rejected}"
            );
        }
    }
}
