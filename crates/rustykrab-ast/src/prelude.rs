//! Common imports for RustyKrab AST.
//!
//! Importing the prelude brings the most commonly used
//! AST types into scope.
//!
//! Example:
//!
//! ```rust
//! use rustykrab_ast::prelude::*;
//! ```
//

// ============================================================
// AST
// ============================================================

pub use crate::ast::{Ast, AstVersion};

// ============================================================
// Node
// ============================================================

pub use crate::node::Node;

// ============================================================
// Node Kinds
// ============================================================

pub use crate::kinds::NodeKind;

// ============================================================
// IDs
// ============================================================

pub use crate::ids::NodeId;

// ============================================================
// Metadata
// ============================================================

pub use crate::metadata::{CompilerMetadata, DiagnosticTag, Metadata, SourceLocation};

// ============================================================
// Property System
// ============================================================

pub use crate::property::{
    PropertyAccess, PropertyKeys, PropertyMap, PropertyObject, PropertyValue,
};

// ============================================================
// Property Validation
// ============================================================

pub use crate::property::{
    has_property, require_boolean_property, require_integer_property, require_property,
    require_string_property,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prelude_exports_work() {
        let node = Node::new(NodeId::new("text"), NodeKind::Text)
            .property(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

        assert!(node.get(PropertyKeys::VALUE).is_some());
    }
}
