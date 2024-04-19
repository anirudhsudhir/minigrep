use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filepath: String,
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

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let filecontent = fs::read_to_string(config.filepath.clone())?;

    for line in search(&config.query, &filecontent) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "help";
        let content = "\
I can’t help thinking:
Why is “abbreviation”
A very long word?";
        assert_eq!(vec!["I can’t help thinking:"], search(query, content));
    }
}
