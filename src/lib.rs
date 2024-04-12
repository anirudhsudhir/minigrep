use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let filecontent = fs::read_to_string(config.filepath.clone())?;
    println!("{}", filecontent);

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("query string and filepath arguments are required")
        } else {
            Ok(Config {
                query: args[1].clone(),
                filepath: args[2].clone(),
            })
        }
    }
}
