//! RustyKrab Prelude
//!
//! Convenient re-exports for common RustyKrab AST types.
//!
//! Instead of importing each module individually:
//!
//! ```ignore
//! use rustykrab_ast::ast::Ast;
//! use rustykrab_ast::ids::NodeId;
//! use rustykrab_ast::kinds::NodeKind;
//! ```
//!
//! Users can simply:
//!
//! ```ignore
//! use rustykrab_ast::prelude::*;
//! ```

// ============================================================
// AST
// ============================================================

pub use crate::ast::{Ast, AstVersion};

// ============================================================
// IDs
// ============================================================

pub use crate::ids::NodeId;

// ============================================================
// Node Kinds
// ============================================================

pub use crate::kinds::NodeKind;

// ============================================================
// Metadata
// ============================================================

pub use crate::metadata::{Metadata, SourceLocation};

// ============================================================
// Nodes
// ============================================================

pub use crate::node::Node;

// ============================================================
// Property System
// ============================================================

pub use crate::property::{
    PropertyAccess, PropertyKeys, PropertyMap, PropertyObject, PropertyValue,
};

// ============================================================
// Modifier System
// ============================================================

pub use crate::modifier::{
    Modifier, ModifierBuilders, ModifierCategory, ModifierChain, ModifierKind, ModifierLike,
    ModifierNames, ModifierValue, ValidationResult, validate_corner_radius, validate_modifier,
    validate_opacity, validate_padding,
};

// ============================================================
// Prelude Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prelude_exports_work() {
        let node = Node::new(NodeId::new("text"), NodeKind::Text)
            .property(PropertyKeys::VALUE, PropertyValue::String("Hello".into()));

        assert!(node.get(PropertyKeys::VALUE,).is_some());
    }

    #[test]
    fn modifier_builder_exports_work() {
        let node = Node::new(NodeId::new("title"), NodeKind::Text)
            .padding(16)
            .background("#FF0000")
            .corner_radius(8);

        assert_eq!(node.modifier_count(), 3,);
    }

    #[test]
    fn prelude_exports_modifier_types() {
        let modifier = Modifier::new(ModifierKind::Padding, ModifierValue::Integer(16));

        assert_eq!(modifier.kind(), &ModifierKind::Padding,);
    }

    #[test]
    fn prelude_exports_modifier_chain() {
        let node = Node::new(NodeId::new("title"), NodeKind::Text)
            .padding(16)
            .opacity(0.8);

        assert_eq!(node.modifier_count(), 2,);

        assert!(node.has_modifiers(),);
    }

    #[test]
    fn prelude_exports_validation() {
        let modifier = Modifier::new(ModifierKind::Padding, ModifierValue::Integer(16));

        assert!(validate_modifier(&modifier,).is_ok());
    }

    #[test]
    fn prelude_exports_property_keys() {
        assert_eq!(PropertyKeys::VALUE, "value",);
    }

    #[test]
    fn prelude_exports_modifier_constants() {
        assert_eq!(ModifierNames::PADDING, "padding",);
    }

    #[test]
    fn prelude_exports_modifier_categories() {
        assert_eq!(ModifierCategory::Layout.as_str(), "layout",);
    }
}
