use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error reading arguments: {}", err);
        process::exit(1);
    });

    println! {"query = {}, filepath = {}",config.query, config.filepath};

    if let Err(err) = minigrep::run(&config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
