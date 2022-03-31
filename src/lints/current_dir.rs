use crate::{FileType, Lint, Match};

use tree_sitter::Node;

pub struct CurrentDir {}

impl Lint for CurrentDir {
    fn file_type(&self) -> FileType {
        FileType::RustSource
    }

    // TODO: Properly resolve the function that is being called in order to _really_ check if the
    // function is `std::env::current_dir`.
    fn matches(&self, node: &Node, source: &[u8]) -> Option<Box<dyn Match>> {
        if node.kind() != "call_expression" {
            return None;
        }

        let function: Node = node.child_by_field_name("function").unwrap();
        if function.kind() != "scoped_identifier" {
            return None;
        }

        let path: Node = function.child_by_field_name("path").unwrap();
        let path: &str = path.utf8_text(source).unwrap();
        if path != "env" {
            return None;
        }

        let name: Node = function.child_by_field_name("name").unwrap();
        let name: &str = name.utf8_text(source).unwrap();
        if name != "current_dir" {
            return None;
        }

        Some(Box::new(CurrentDirMatch::new()))
    }
}

struct CurrentDirMatch {}

impl CurrentDirMatch {
    pub fn new() -> Self {
        Self {}
    }
}

impl Match for CurrentDirMatch {
    fn message(&self) -> String {
        String::from("`std::env::current_dir` calls `getcwd()` on Linux which canonicalizes paths by resolving `.`, `..`, and symlinks. This might not be the desired behavior.")
    }
}
