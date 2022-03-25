pub use crate::run_result::RunResult;
use crate::Options;

use std::fs;
use std::path::PathBuf;

use walkdir::WalkDir;

use tree_sitter::{Language, Node, Parser, Point, Tree, TreeCursor};
use tree_sitter_bash;

pub fn run(options: Options) -> RunResult {
    let mut errors = 0;

    let mut parser = Parser::new();

    let language: Language = tree_sitter_bash::language();
    parser.set_language(language).unwrap();

    let bash_files = bash_files(options.paths);
    for bash_file in bash_files {
        let source = fs::read_to_string(&bash_file).unwrap();
        let source_bytes: &[u8] = source.as_bytes();

        let tree: Tree = parser.parse(&source, None).unwrap();

        let cursor: TreeCursor = tree.walk();
        let walk: Walk = Walk::from(cursor);

        for node in walk {
            if apt_call(&node, source_bytes) {
                let start: Point = node.start_position();
                let relative_path = bash_file
                    .strip_prefix(&options.current_dir)
                    .unwrap()
                    .to_string_lossy();
                println!(
                    "{}:{},{} error: `apt` call in a Bash script.",
                    relative_path, start.row, start.column
                );
                errors += 1;
            }
        }
    }

    RunResult::new(errors)
}

fn bash_files(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let path_iterators = paths.iter().map(|path| {
        WalkDir::new(path)
            .into_iter()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .ends_with(".sh")
            })
            .map(|entry| entry.unwrap().path().to_path_buf())
            .collect()
    });

    path_iterators
        .reduce(|mut accumulated: Vec<PathBuf>, paths| {
            accumulated.extend(paths);
            accumulated
        })
        .unwrap_or_default()
}

/// Return true if the node is a bash command which calls apt.
fn apt_call(node: &Node, source: &[u8]) -> bool {
    if node.kind() != "command" {
        return false;
    }

    let name: Node = node.child_by_field_name("name").unwrap();
    let name = name.utf8_text(source).unwrap();

    if name == "apt" {
        return true;
    }

    if name == "sudo" {
        match node.child_by_field_name("argument") {
            Some(node) => node.utf8_text(source).unwrap() == "apt",
            None => false,
        }
    } else {
        false
    }
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
