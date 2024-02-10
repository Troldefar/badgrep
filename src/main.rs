use std::env;
use std::process;

use minigrep::FileConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_config: FileConfig = FileConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(file_config) {
        eprintln!("Error {e}");
        process::exit(1);
    }
}

