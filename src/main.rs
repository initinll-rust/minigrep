use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args = env::args();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problems parsing arguments : {err}");
        process::exit(1);
    });

    println!("\nQuery: {}", config.query);
    println!("\nFile: {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}