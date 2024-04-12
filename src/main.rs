use std::{env, fs};

struct Config {
    query: String,
    filepath: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    println! {"query = {}, filepath = {}",config.query, config.filepath};

    let filecontent = fs::read_to_string(config.filepath).expect("Error reading file");

    println!("{}", filecontent);
}

fn parse_args(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        filepath: args[2].clone(),
    }
}
