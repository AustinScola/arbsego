use arbsego::{self, Options, RunResult};

use std::env;
use std::path::PathBuf;
use std::process;

use clap::Parser;

use env_logger::Builder;

use log::LevelFilter as LogLevel;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Enable debug logging
    #[clap(long)]
    debug: bool,

    /// Paths to run on
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let log_level = match args.debug {
        true => LogLevel::Debug,
        false => LogLevel::Off,
    };
    set_up_logging(log_level);

    let current_dir = env::current_dir().unwrap(); // arbsego: ignore

    let paths: Vec<PathBuf> = if args.paths.is_empty() {
        vec![current_dir.clone()]
    } else {
        args.paths.iter().map(PathBuf::from).collect()
    };

    let options = Options::new(current_dir, paths);

    let result: RunResult = arbsego::run(options);
    if result.errors > 0 {
        process::exit(1)
    }
}

/// Set up logging.
fn set_up_logging(level: LogLevel) {
    Builder::new().filter_level(level).init();
}
