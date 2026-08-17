//! Retained lab for exact-fact concurrent Entity requests.
//!
//! This crate is deliberately independent from production Aicadia. It tests a
//! bounded request package against real PostgreSQL transactions without selecting a
//! production API, schema, admission policy or game mechanic.

pub mod world;
