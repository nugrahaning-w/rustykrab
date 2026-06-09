#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prelude_exports_work() {
        let root = Node::new(
            NodeId::new("root"),
            NodeKind::VStack,
        );

        let ast = Ast::new(root);

        assert_eq!(
            ast.version,
            AstVersion::current()
        );
    }
}