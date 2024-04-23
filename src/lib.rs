use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_insensitive_var: bool,
    pub case_insensitive_flag: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("query string and filepath arguments are required")
        } else {
            Ok(Config {
                query: args[1].clone(),
                filepath: args[2].clone(),
                case_insensitive_var: env::var("IGNORE_CASE").is_ok(),
                case_insensitive_flag: args.len() > 3 && args[3] == "-i",
            })
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let filecontent = fs::read_to_string(config.filepath.clone())?;

    let results = if config.case_insensitive_flag || config.case_insensitive_var {
        search_insensitive(&config.query, &filecontent)
    } else {
        search_sensitive(&config.query, &filecontent)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_sensitive() {
        let query = "w";
        let content = "\
I can’t help thinking:
Why is “abbreviation”
A very long word?";
        assert_eq!(vec!["A very long word?"], search_sensitive(query, content));
    }

    #[test]
    fn test_search_insensitive() {
        let query = "w";
        let content = "\
I can’t help thinking:
Why is “abbreviation”
A very long word?";
        assert_eq!(
            vec!["Why is “abbreviation”", "A very long word?"],
            search_insensitive(query, content)
        );
    }
}
