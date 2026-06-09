//! Common imports for RustyKrab AST.
//!
//! Importing the prelude brings the most commonly used
//! AST types into scope.
//!
//! Example:
//!
//! use rustykrab_ast::prelude::*;
//!

// AST
pub use crate::ast::{Ast, AstVersion};

// Node
pub use crate::node::Node;

// Node Kinds
pub use crate::kinds::NodeKind;

// IDs
pub use crate::ids::NodeId;

// Metadata
pub use crate::metadata::{CompilerMetadata, DiagnosticTag, Metadata, SourceLocation};
