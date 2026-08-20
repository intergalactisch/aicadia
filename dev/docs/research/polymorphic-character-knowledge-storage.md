---
status: load-bearing
era: August 2026 spatial S1
---

# Laravel-style polymorphic Character knowledge storage

> **Role / side:** sourced storage-mechanism research / development side.
> **Authority:** records what current Laravel and PostgreSQL guarantee for one
> Character-to-several-target-type association table and the resulting S1 boundary.
> **Excludes:** the product choice between generic and typed Knowledge storage and
> any current schema or behavior; those belong in the active concept record, plan
> and `game/docs/` after acceptance.

Controlled: 2026-08-20

Status: current primary-source finding; the S1 storage choice remains open

## Question

Can a Laravel-like polymorphic table reliably and scalably remember that one
Character knows a World record when the current target is either a Place or a
Connection?

The narrow question is about storage mechanics. It does not decide whether Place
Knowledge and Connection Knowledge have the same game meaning or lifecycle.

## Evidence boundary

Laravel 13.x documents and implements a mature framework convention. It proves the
column and lookup shape is ordinary application practice; it does not prove
PostgreSQL referential integrity across several target tables or Aicadia-scale query
behavior. PostgreSQL 18 documentation establishes the database guarantees below.
The Aicadia implications are reasoned consequences, not accepted product choices.

## Short answer

Yes, the polymorphic lookup shape can be reliable and efficient for bounded S1
queries when its composite key follows those queries. It can enforce one row per
Character and typed target. The limitation is separate: a `(subject_type,
subject_id)` pair cannot use one native PostgreSQL foreign key that chooses the
referenced table from `subject_type`.

Indexes therefore solve lookup cost and duplicate admission. They do not establish
that `subject_id` names a real Place when the type is `place`, or a real Connection
when the type is `connection`.

## 1. What Laravel actually creates

Laravel's `morphs('taggable')` convention creates `taggable_type` and
`taggable_id`. The id column follows the model key type. Laravel's current
`Blueprint` source creates a non-unique index ordered as `(taggable_type,
taggable_id)` for numeric, UUID and ULID variants.
[Laravel 13.x migrations, `morphs()`](https://laravel.com/docs/13.x/migrations#column-method-morphs),
[Laravel 13.x `Blueprint` source](https://github.com/laravel/framework/blob/13.x/src/Illuminate/Database/Schema/Blueprint.php#L1571-L1706)

That default index is shaped for finding rows which point at one typed target. It
does not include an owning Character, impose uniqueness or add a foreign key to the
polymorphic target. Those guarantees must be designed separately for an association
table.

Laravel also stores a fully qualified model class in the type column by default.
Its morph-map facility replaces class names with stable aliases such as `post` and
`video`, explicitly so stored values survive class renames.
[Laravel 13.x Eloquent relationships, custom polymorphic types](https://laravel.com/docs/13.x/eloquent-relationships#custom-polymorphic-types)

The transferable lesson for Aicadia is only mechanical: if a polymorphic shape is
chosen, persisted type values should be stable game/server aliases such as the two
current candidates `place` and `connection`, not Rust module or struct names. This
does not by itself accept a generic Knowledge model.

## 2. Index order must follow the S1 read

PostgreSQL can use a multicolumn B-tree index for conditions on any subset of its
columns, but it is most efficient when equality conditions constrain its leading
columns. Conditions only on later columns may leave a large part of the index to
scan. PostgreSQL also cautions that multicolumn indexes should be added for an actual
query shape rather than by default.
[PostgreSQL 18, multicolumn indexes](https://www.postgresql.org/docs/current/indexes-multicolumn.html)

For the current Character-scoped eligibility read, the relevant candidate key is
therefore not Laravel's target-first index. This shape is illustrative and
non-canonical:

```sql
PRIMARY KEY (character_entity_id, subject_type, subject_id)
```

It supports both current bounded predicates through the leftmost prefix:

```sql
-- Illustrative only: one Character's eligible records, optionally one current type.
WHERE character_entity_id = $1
  AND subject_type = $2
ORDER BY subject_id
LIMIT $3

-- Illustrative only: exact eligibility check.
WHERE character_entity_id = $1
  AND subject_type = $2
  AND subject_id = $3
```

The same primary key supports a Character read without a type filter because
`character_entity_id` remains the leading equality column. If a later accepted
operation asks the reverse question—every Character who knows one target—it would
have a different leading predicate and would need its own target-first index. The
current S1 query does not receive that index merely because Laravel provides one.

## 3. Uniqueness is a database guarantee

PostgreSQL permits a primary key or unique constraint across several columns and
automatically implements it with a unique B-tree index. A primary key additionally
makes every key column non-null.
[PostgreSQL 18, unique constraints and primary keys](https://www.postgresql.org/docs/current/ddl-constraints.html#DDL-CONSTRAINTS-UNIQUE-CONSTRAINTS)

Consequently, the illustrative three-column primary key can deterministically
prevent duplicate rows for the same Character, type and target, including two
concurrent attempts that reach the unique constraint. This is stronger than
Laravel's default non-unique morph index. It does not require a surrogate Knowledge
id for uniqueness.

Rows for different Characters or different targets have different keys; the shape
introduces no single application-owned counter, global coordination row or
Character-wide write lock. PostgreSQL documentation supports the B-tree lookup and
uniqueness claims, but does not supply a universal “millions of players” capacity
guarantee. Table size, row width, bounded pagination, actual query plans and write
load would still require production-like measurement.

## 4. Why the target cannot have one dynamic native foreign key

A PostgreSQL foreign key names one referenced table and one referenced column set:

```sql
FOREIGN KEY (...) REFERENCES one_table (...)
```

It guarantees that the referencing values match a row in that named table.
[PostgreSQL 18, foreign keys](https://www.postgresql.org/docs/current/ddl-constraints.html#DDL-CONSTRAINTS-FK)

There is no native foreign-key syntax in which `subject_type = 'place'` selects the
Place table while `subject_type = 'connection'` selects the Connection table. A
`CHECK` constraint can restrict `subject_type` to those two values, but PostgreSQL
explicitly does not support `CHECK` constraints that safely depend on rows in other
tables.
[PostgreSQL 18, check-constraint cross-table boundary](https://www.postgresql.org/docs/current/ddl-constraints.html#DDL-CONSTRAINTS-CHECK-CONSTRAINTS)

Therefore a generic row can keep ordinary native foreign keys on its fixed owner or
provenance columns, but the polymorphic target needs one of these distinct integrity
mechanisms:

- deterministic World validation and typed target lookup, with every accepted
  writer following the same transactional rule;
- custom trigger machinery that also accounts for target updates and deletion;
- separate typed association tables, each with its own native target foreign key;
  or
- one shared target registry which all eligible target records reference, making
  that registry the single native foreign-key target.

These mechanisms have different lifecycle and complexity consequences. The sources
do not choose between them. Application validation alone can be bypassed by a faulty
manual writer, and validation must be synchronized with any target deletion or
identity change to avoid a race. Native foreign keys perform that continuing
referential-integrity work inside PostgreSQL.

## 5. Exact implication for current S1

S1 currently presents precisely two possible target types: Place and Connection.
For that boundary, a Laravel-like table is technically viable if all of the
following are made explicit in the later product choice:

1. The Character is the fixed owner; `character_entity_id` leads the key used by
   bounded Character-scoped reads.
2. `subject_type` admits only the current `place` and `connection` values; it is not
   a stored language/runtime class name or an open arbitrary string.
3. `(character_entity_id, subject_type, subject_id)` is unique and non-null.
4. Hydration dispatches deterministically by the stored type and batches at most the
   two current target-table reads; prose performs no type inference.
5. The chosen integrity mechanism proves that the typed target exists in the same
   accepted write boundary and states what happens if targets can later disappear.
6. Reads stay bounded and paginated. A reverse target-to-all-Characters index is
   absent unless S1 actually needs that query.

This establishes feasibility, not equivalence. One table is justified only if the
event which establishes Knowledge, the meaning of its row, its provenance, removal
rules and read eligibility truly agree for Place and Connection. If those semantics
differ, indexes cannot make the domain records interchangeable.

## Findings carried into the open choice

- **Corrected:** a polymorphic table is not inherently slow or unserious. A
  Character-leading composite primary key is a conventional bounded lookup and
  uniqueness mechanism.
- **Preserved boundary:** a type-plus-id index does not provide a dynamic target
  foreign key.
- **Laravel comparison:** Laravel supplies type/id columns, a target-first
  non-unique index and optional stable morph aliases; Aicadia would still own its
  Character key, uniqueness, target integrity, lifecycle and query bounds.
- **Scale statement:** the shape contains no inherent global hot row, but only
  query-plan and load evidence can establish a concrete capacity claim.
- **Decision still open:** current evidence does not select a generic table, two
  typed tables, a registry or custom integrity machinery.

## Primary sources

- [Laravel 13.x — Database: Migrations](https://laravel.com/docs/13.x/migrations#column-method-morphs)
- [Laravel Framework 13.x — `Blueprint.php`](https://github.com/laravel/framework/blob/13.x/src/Illuminate/Database/Schema/Blueprint.php#L1571-L1706)
- [Laravel 13.x — Eloquent: Relationships](https://laravel.com/docs/13.x/eloquent-relationships#custom-polymorphic-types)
- [PostgreSQL 18 — Multicolumn Indexes](https://www.postgresql.org/docs/current/indexes-multicolumn.html)
- [PostgreSQL 18 — Constraints](https://www.postgresql.org/docs/current/ddl-constraints.html)
