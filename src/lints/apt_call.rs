use crate::{FileType, Lint, Match};

use tree_sitter::Node;

pub struct AptCall {}

impl Lint for AptCall {
    fn file_type(&self) -> FileType {
        FileType::BashSource
    }

    /// Return if the node is a bash command which calls apt.
    fn r#match(&self, node: &Node, source: &[u8]) -> Option<Box<dyn Match>> {
        if node.kind() != "command" {
            return None;
        }

        let name: Node = node.child_by_field_name("name").unwrap();
        let name = name.utf8_text(source).unwrap();

        if name == "apt" {
            return Some(Box::new(AptCallMatch {}));
        }

        if name == "sudo" {
            match node.child_by_field_name("argument") {
                Some(node) => {
                    if node.utf8_text(source).unwrap() == "apt" {
                        Some(Box::new(AptCallMatch {}))
                    } else {
                        None
                    }
                }
                None => None,
            }
        } else {
            None
        }
    }
}

struct AptCallMatch {}

impl Match for AptCallMatch {
    fn message(&self) -> String {
        String::from("`apt` call in a Bash script.")
    }
}
