use crate::{FileType, Match};

use tree_sitter::Node;

pub trait Pattern: Sync {
    fn file_type(&self) -> FileType;

    fn r#match(&self, node: &Node, source: &[u8]) -> Option<Box<dyn Match>>;
}
