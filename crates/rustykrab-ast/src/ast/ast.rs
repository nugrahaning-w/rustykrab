use crate::{ast::AstVersion, node::Node};

/// Root object of the RustyKrab AST.
///
/// Every RustyKrab application produces exactly
/// one AST root.
///
/// The AST acts as the boundary between:
///
/// - Parser
/// - Validator
/// - Generator
/// - Tooling
///
/// Generators must consume Ast instead of
/// directly consuming Node.
#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub version: AstVersion,

    pub root: Node,
}

impl Ast {
    /// Creates a new AST using the current
    /// supported AST version.
    pub fn new(root: Node) -> Self {
        Self {
            version: AstVersion::current(),
            root,
        }
    }

    /// Creates a new AST with an explicit
    /// version.
    pub fn with_version(version: AstVersion, root: Node) -> Self {
        Self { version, root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ids::NodeId, kinds::NodeKind};

    #[test]
    fn create_ast() {
        let root = Node::new(NodeId::new("root"), NodeKind::VStack);

        let ast = Ast::new(root.clone());

        assert_eq!(ast.version, AstVersion::current());

        assert_eq!(ast.root, root);
    }

    #[test]
    fn create_ast_with_version() {
        let root = Node::new(NodeId::new("root"), NodeKind::VStack);

        let version = AstVersion::new(1, 0);

        let ast = Ast::with_version(version, root.clone());

        assert_eq!(ast.version, version);

        assert_eq!(ast.root, root);
    }
}
