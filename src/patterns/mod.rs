mod apt_call;
pub use apt_call::AptCall;

mod current_dir;
pub use current_dir::CurrentDir;

mod rust_comment_line_ignore;
pub use rust_comment_line_ignore::RustCommentLineIgnore;

use super::pattern::Pattern;

lazy_static! {
    pub static ref BASH_LINTS: Vec<Box<dyn Pattern>> = vec![Box::new(AptCall {})];
    pub static ref RUST_LINTS: Vec<Box<dyn Pattern>> = vec![Box::new(CurrentDir {})];
    pub static ref OTHER_LINTS: Vec<Box<dyn Pattern>> = vec![];
    pub static ref RUST_LINE_IGNORE: Box<dyn Pattern> = Box::new(RustCommentLineIgnore {});
}
