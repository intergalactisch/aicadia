//! Bounded keyset paging shared by every Studio Live read.
//!
//! Studio never asks PostgreSQL for an unbounded result. One read declares a
//! [`Bound`], asks for exactly one row more than that bound and turns the answer
//! into a [`Page`] that names both the cursor continuing the keyset and whether
//! more rows existed. The bound is the same for every subject, so a hot Place,
//! a hot Entity and an empty World all cost the same worst case.

use serde::Serialize;

use crate::studio::StudioError;

/// The largest page any Studio Live read returns.
pub const MAX_LIMIT: u16 = 100;
/// The page size a Studio Live read uses when the operator names none.
pub const DEFAULT_LIMIT: u16 = 24;

/// One validated Studio read bound between one and [`MAX_LIMIT`] rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bound(usize);

impl Bound {
    /// Validates an operator-supplied limit, defaulting to [`DEFAULT_LIMIT`].
    pub fn new(limit: Option<u16>) -> Result<Self, StudioError> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT);
        if !(1..=MAX_LIMIT).contains(&limit) {
            return Err(StudioError::InvalidLimit);
        }
        Ok(Self(usize::from(limit)))
    }

    /// The number of rows the caller may present.
    pub fn limit(self) -> usize {
        self.0
    }

    /// The `LIMIT` to bind: one row more than the bound, to detect a next page.
    pub fn fetch(self) -> i64 {
        i64::try_from(self.0 + 1).expect("a Studio page bound always fits i64")
    }
}

impl Default for Bound {
    fn default() -> Self {
        Self(usize::from(DEFAULT_LIMIT))
    }
}

/// One bounded keyset page: the visible rows, the cursor that continues the
/// keyset after the last visible row and whether more rows existed.
#[derive(Clone, Debug, Serialize)]
pub struct Page<T, C> {
    pub item: Vec<T>,
    pub next_cursor: Option<C>,
    pub truncated: bool,
}

impl<T, C> Page<T, C> {
    /// Turns one [`Bound::fetch`]-sized read into a page of at most `limit` rows.
    pub fn build(mut item: Vec<T>, bound: Bound, cursor: impl FnOnce(&T) -> C) -> Self {
        let truncated = item.len() > bound.limit();
        item.truncate(bound.limit());
        let next_cursor = match (truncated, item.last()) {
            (true, Some(last)) => Some(cursor(last)),
            _ => None,
        };
        Self {
            item,
            next_cursor,
            truncated,
        }
    }

    /// One page that carries no keyset continuation, only a truncation flag.
    pub fn unordered(mut item: Vec<T>, bound: Bound) -> Self {
        let truncated = item.len() > bound.limit();
        item.truncate(bound.limit());
        Self {
            item,
            next_cursor: None,
            truncated,
        }
    }
}

/// Cuts one bounded preview list read with `limit + 1` rows down to `limit` and
/// reports whether at least one further row existed.
pub fn truncate<T>(item: &mut Vec<T>, limit: usize) -> bool {
    let truncated = item.len() > limit;
    item.truncate(limit);
    truncated
}

/// The label a read carries when PostgreSQL cannot serve it from an index, and
/// which is therefore affordable only because a local development World is
/// small. A page showing this label must never present the read as a game read.
pub const LOCAL_DEVELOPMENT_SCAN: &str = "local development scan";

/// The bound every Studio preview count uses: counting stops at `limit + 1`, so
/// a page can honestly render "100+" instead of paying for an exact aggregate.
pub const PREVIEW_COUNT_LIMIT: i64 = 101;

/// Splits a bounded count read into the presentable count and its truncation flag.
pub fn preview_count(counted: i64) -> (i64, bool) {
    let truncated = counted >= PREVIEW_COUNT_LIMIT;
    (counted.min(PREVIEW_COUNT_LIMIT - 1), truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_studio_bound_defaults_to_twenty_four_and_never_exceeds_one_hundred() {
        assert_eq!(Bound::new(None).unwrap().limit(), 24);
        assert_eq!(Bound::new(Some(100)).unwrap().limit(), 100);
        assert_eq!(Bound::new(Some(1)).unwrap().fetch(), 2);
        assert!(matches!(
            Bound::new(Some(0)),
            Err(StudioError::InvalidLimit)
        ));
        assert!(matches!(
            Bound::new(Some(101)),
            Err(StudioError::InvalidLimit)
        ));
        assert_eq!(Bound::default().limit(), usize::from(DEFAULT_LIMIT));
    }

    #[test]
    fn a_page_cursor_is_the_last_visible_row_only_when_another_row_exists() {
        let bound = Bound::new(Some(2)).unwrap();

        let full = Page::build(vec![1, 2, 3], bound, |item| *item);
        assert_eq!(full.item, vec![1, 2]);
        assert_eq!(full.next_cursor, Some(2));
        assert!(full.truncated);

        let exact = Page::build(vec![1, 2], bound, |item| *item);
        assert_eq!(exact.item, vec![1, 2]);
        assert_eq!(exact.next_cursor, None);
        assert!(!exact.truncated);

        let empty = Page::build(Vec::<i32>::new(), bound, |item| *item);
        assert!(empty.item.is_empty());
        assert_eq!(empty.next_cursor, None);
        assert!(!empty.truncated);
    }

    #[test]
    fn an_unordered_page_reports_truncation_without_a_cursor() {
        let bound = Bound::new(Some(1)).unwrap();
        let page = Page::<i32, ()>::unordered(vec![7, 8], bound);

        assert_eq!(page.item, vec![7]);
        assert_eq!(page.next_cursor, None);
        assert!(page.truncated);
    }

    #[test]
    fn a_preview_count_stops_at_one_hundred_and_says_so() {
        assert_eq!(preview_count(0), (0, false));
        assert_eq!(preview_count(100), (100, false));
        assert_eq!(preview_count(101), (100, true));

        let mut item = vec![1, 2, 3];
        assert!(truncate(&mut item, 2));
        assert_eq!(item, vec![1, 2]);
        assert!(!truncate(&mut item, 2));
    }
}
