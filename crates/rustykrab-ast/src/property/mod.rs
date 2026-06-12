//! Property System
//!
//! The Property System provides dynamic,
//! extensible configuration storage for AST nodes.
//!
//! Main Components:
//!
//! - PropertyValue
//! - PropertyMap
//! - PropertyObject
//! - PropertyAccess
//! - PropertyKeys
//!
//! Example:
//!
//! ```rust
//! use rustykrab_ast::prelude::*;
//!
//! let node = Node::new(
//!     NodeId::new("text"),
//!     NodeKind::Text,
//! )
//! .property(
//!     PropertyKeys::VALUE,
//!     PropertyValue::String(
//!         "Hello".into(),
//!     ),
//! );
//! ```
mod keys;
mod property_access;
mod property_map;
mod property_object;
mod property_value;
mod validation;

pub use keys::PropertyKeys;
pub use property_access::PropertyAccess;
pub use property_map::PropertyMap;
pub use property_object::PropertyObject;
pub use property_value::PropertyValue;
pub use validation::*;

#[cfg(test)]
mod tests;
