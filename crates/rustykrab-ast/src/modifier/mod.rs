//! Modifier System
//!
//! The Modifier System provides platform-independent
//! styling, layout, visual effects, accessibility,
//! and animation capabilities for AST nodes.
//!
//! Modifiers are stored inside a ModifierChain and
//! attached to Nodes.
//!
//! Example:
//!
//! ```ignore
//! let node = Node::new(
//!     NodeId::new("title"),
//!     NodeKind::Text,
//! )
//! .padding(16)
//! .background("#FF0000")
//! .corner_radius(8)
//! .opacity(0.8);
//! ```
//!
//! This module serves as the foundation for:
//!
//! - SwiftUI Generation
//! - Jetpack Compose Generation
//! - Future Web Renderers
//! - AST Validation
//! - Plugin Systems

// ============================================================
// Internal Modules
// ============================================================

mod builders;
mod categories;
mod constants;
mod modifier;
mod modifier_chain;
mod modifier_kind;
mod modifier_trait;
mod modifier_value;
mod validation;

// ============================================================
// Builder APIs
// ============================================================

pub use builders::ModifierBuilders;

// ============================================================
// Categories
// ============================================================

pub use categories::ModifierCategory;

// ============================================================
// Constants
// ============================================================

pub use constants::ModifierNames;

// ============================================================
// Core Models
// ============================================================

pub use modifier::Modifier;
pub use modifier_chain::ModifierChain;
pub use modifier_kind::ModifierKind;
pub use modifier_value::ModifierValue;

// ============================================================
// Traits
// ============================================================

pub use modifier_trait::ModifierLike;

// ============================================================
// Validation
// ============================================================

pub use validation::{
    ValidationResult, validate_corner_radius, validate_modifier, validate_opacity, validate_padding,
};

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests;
