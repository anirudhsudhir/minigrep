use minigrep::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error reading arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(&config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
