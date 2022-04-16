use crate::lints::{AptCall, CurrentDir};
pub use crate::run_result::RunResult;
use crate::{File, FileType, Languages, Lint, Options};

use std::fs;
use std::path::PathBuf;

use log::debug;

use walkdir::WalkDir;

use tree_sitter::{Language, Node, Parser, Point, Tree, TreeCursor};

pub fn run(options: Options) -> RunResult {
    let mut errors = 0;

    let mut parser = Parser::new();

    let languages = Languages::new();

    let bash_lints: Vec<Box<dyn Lint>> = vec![Box::new(AptCall {})];
    let rust_lints: Vec<Box<dyn Lint>> = vec![Box::new(CurrentDir {})];
    let other_lints: Vec<Box<dyn Lint>> = vec![];

    let files: Vec<File> = files(options.paths)
        .into_iter()
        .filter(|file| file.file_type().has_lints())
        .collect();
    for file in files {
        debug!("Linting file \"{:?}\"...", file.path());

        let language: Language = languages.language_from_file_type(file.file_type()).unwrap();
        parser.set_language(language).unwrap();

        let source = fs::read_to_string(&file).unwrap();
        let source_bytes: &[u8] = source.as_bytes();

        let tree: Tree = parser.parse(&source, None).unwrap();

        let cursor: TreeCursor = tree.walk();
        let walk: Walk = Walk::from(cursor);

        let lints: &Vec<Box<dyn Lint>> = match file.file_type() {
            FileType::BashSource => &bash_lints,
            FileType::RustSource => &rust_lints,
            FileType::Other => &other_lints,
        };

        for node in walk {
            for lint in lints {
                if let Some(r#match) = lint.r#match(&node, source_bytes) {
                    let start: Point = node.start_position();
                    let start_row = start.row;
                    let start_column = start.column;
                    let relative_path = file
                        .path()
                        .strip_prefix(&options.current_dir)
                        .unwrap()
                        .to_string_lossy();
                    let message = r#match.message();
                    println!("{relative_path}:{start_row},{start_column} error: {message}");
                    errors += 1;
                }
            }
        }

        debug!("Linted file \"{:?}\".", file.path());
    }

    RunResult::new(errors)
}

fn files(paths: Vec<PathBuf>) -> Vec<File> {
    let file_iterators = paths.iter().map(|path| {
        let files = WalkDir::new(path)
            .into_iter()
            .map(|entry| entry.unwrap().path().to_path_buf())
            .filter(|path| path.is_file());
        files.map(File::from).collect()
    });

    file_iterators
        .reduce(|mut accumulated: Vec<File>, paths| {
            accumulated.extend(paths);
            accumulated
        })
        .unwrap_or_default()
}

struct Walk<'a> {
    cursor: TreeCursor<'a>,
    done: bool,
}

impl<'a> From<TreeCursor<'a>> for Walk<'a> {
    fn from(cursor: TreeCursor<'a>) -> Self {
        Self {
            cursor,
            done: false,
        }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let node = self.cursor.node();

        match self.cursor.goto_first_child() {
            true => {}
            false => loop {
                match self.cursor.goto_next_sibling() {
                    true => {
                        break;
                    }
                    false => match self.cursor.goto_parent() {
                        false => {
                            self.done = true;
                            break;
                        }
                        true => {
                            continue;
                        }
                    },
                }
            },
        }

        Some(node)
    }
}
