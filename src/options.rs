use std::path::PathBuf;

pub struct Options {
    pub current_dir: PathBuf,
    pub paths: Vec<PathBuf>,
}

impl Options {
    pub fn new(current_dir: PathBuf, paths: Vec<PathBuf>) -> Self {
        Self { current_dir, paths }
    }
}
