#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! RustyKrab AST
//!
//! Platform-independent Abstract Syntax Tree (AST)
//! used by RustyKrab generators, validators,
//! parsers, and tooling.
//!
//! This crate provides:
//!
//! - AST Root
//! - Node Model
//! - NodeKind System
//! - Metadata System
//! - Tree Manipulation APIs
//!
//! Most users should import:
//!
//! ```rust
//! use rustykrab_ast::prelude::*;
//! ```
pub mod ast;
pub mod event;
pub mod ids;
pub mod kinds;
pub mod metadata;
pub mod modifier;
pub mod node;
pub mod prelude;
pub mod property;

#[cfg(test)]
mod tests;
