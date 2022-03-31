use std::path::Path;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileType {
    BashSource,
    RustSource,
    Other,
}

impl From<&Path> for FileType {
    fn from(path: &Path) -> Self {
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".sh") {
            FileType::BashSource
        } else if file_name.ends_with(".rs") {
            FileType::RustSource
        } else {
            FileType::Other
        }
    }
}

impl FileType {
    pub fn has_lints(&self) -> bool {
        self != &Self::Other
    }
}
