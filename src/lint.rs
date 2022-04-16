use crate::{FileType, Match};

use tree_sitter::Node;

pub trait Lint {
    fn file_type(&self) -> FileType;

    fn r#match(&self, node: &Node, source: &[u8]) -> Option<Box<dyn Match>>;
}
