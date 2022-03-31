use crate::FileType;

use tree_sitter::Language;

pub struct Languages {
    bash: Language,
    rust: Language,
}

impl Languages {
    pub fn new() -> Self {
        let bash = tree_sitter_bash::language();
        let rust = tree_sitter_rust::language();
        Self { bash, rust }
    }

    pub fn language_from_file_type(&self, file_type: FileType) -> Option<Language> {
        match file_type {
            FileType::BashSource => Some(self.bash),
            FileType::RustSource => Some(self.rust),
            FileType::Other => None,
        }
    }
}
