use crate::{
    ast::{Ast, AstVersion},
    ids::NodeId,
    kinds::NodeKind,
    node::Node,
};

#[test]
fn create_ast() {
    let root = Node::new(NodeId::new("root"), NodeKind::VStack);

    let ast = Ast::new(root);

    assert_eq!(ast.version, AstVersion::current());
}

#[test]
fn create_ast_with_version() {
    let root = Node::new(NodeId::new("root"), NodeKind::VStack);

    let ast = Ast::with_version(AstVersion::new(1, 0), root);

    assert_eq!(ast.version, AstVersion::new(1, 0));
}
