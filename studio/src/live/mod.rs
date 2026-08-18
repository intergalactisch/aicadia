//! Bounded, operator-only reads over the one connected local World.
//!
//! Every read in this module tree is a pure function over a PostgreSQL pool. It
//! takes an explicit subject and bound, returns projected data and never writes.
//! HTTP presentation belongs to `studio::page`; this module contains no parallel
//! JSON application surface.

pub mod character;
pub mod chronicle;
pub mod entity;
pub mod estimate;
pub mod investigation;
pub mod migration;
pub mod page;
pub mod place;
pub mod property;
pub mod resolve;
pub mod row;
pub mod schema;
pub mod r#trait;
pub mod user;
