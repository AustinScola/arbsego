use crate::FileType;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    path_buf: PathBuf,
    file_type: FileType,
}

impl From<PathBuf> for File {
    fn from(path_buf: PathBuf) -> Self {
        let file_type = FileType::from(path_buf.as_path());
        Self {
            path_buf,
            file_type,
        }
    }
}

impl File {
    pub fn file_type(&self) -> FileType {
        self.file_type
    }
}

impl AsRef<Path> for File {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

impl File {
    pub fn path(&self) -> &Path {
        self.path_buf.as_path()
    }
}
