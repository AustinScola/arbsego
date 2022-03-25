use arbsego::{self, Options, RunResult};

use std::env;
use std::path::PathBuf;
use std::process;

use clap::{arg, command, ArgMatches as Arguments, Command as Parser};

fn main() {
    let parser: Parser = parser();
    let arguments: Arguments = parser.get_matches();

    let current_dir = env::current_dir().unwrap();
    let paths: Vec<PathBuf> = match arguments.values_of("paths") {
        Some(paths) => paths.map(PathBuf::from).collect(),
        None => vec![current_dir.clone()],
    };

    let options = Options::new(current_dir, paths);

    let result: RunResult = arbsego::run(options);
    if result.errors > 0 {
        process::exit(1)
    }
}

/// Return a command line interface (CLI) argument parser.
fn parser() -> Parser<'static> {
    command!().arg(arg!([paths] "Paths to run on").multiple_values(true))
}
