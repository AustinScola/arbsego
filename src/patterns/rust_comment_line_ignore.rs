use crate::{FileType, Match, Pattern};

use tree_sitter::Node;

pub struct RustCommentLineIgnore {}

impl Pattern for RustCommentLineIgnore {
    fn file_type(&self) -> FileType {
        FileType::RustSource
    }

    fn r#match(&self, node: &Node, source: &[u8]) -> Option<Box<dyn Match>> {
        if node.kind() != "line_comment" {
            return None;
        }

        let contents: &str = node.utf8_text(source).unwrap();

        if contents == "// arbsego: ignore" {
            return Some(Box::new(RustCommentLineIgnoreMatch {}));
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RustCommentLineIgnoreMatch {}

impl Match for RustCommentLineIgnoreMatch {
    fn message(&self) -> String {
        // TODO: Having this doesn't really make sense... lets change to separate traits for
        // patterns and lints.
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;
    use tree_sitter::{Language, Node, Parser, Tree};

    #[test_case("// arbsego: ignore", Some(Box::new(RustCommentLineIgnoreMatch {}));)]
    #[test_case("let foo = None;", None;)]
    fn test_match(source: &str, expected_match: Option<Box<dyn Match>>) {
        let mut parser = Parser::new();

        let rust = tree_sitter_rust::language();
        parser.set_language(rust).unwrap();

        let source_bytes: &[u8] = source.as_bytes();

        let tree: Tree = parser.parse(&source, None).unwrap();
        let mut node: Node = tree.root_node();
        node = node.child(0).unwrap();

        let rust_comment_line_ignore: RustCommentLineIgnore = RustCommentLineIgnore {};

        let r#match: Option<Box<dyn Match>> = rust_comment_line_ignore.r#match(&node, source_bytes);

        // TODO: How can we directory compare these? We can't add `PartialEq` as a super trait to
        // `Match` becasuse `ParitalEq` uses `Self` as a parameter.
        assert_eq!(r#match.is_some(), expected_match.is_some());
    }
}
