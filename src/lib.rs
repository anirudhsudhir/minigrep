use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_insensitive_var: bool,
    pub case_insensitive_flag: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Missing arguments: query"),
        };

        let filepath = match args.next() {
            Some(fp) => fp,
            None => return Err("Missing arguments: filepath"),
        };

        let case_insensitive_flag = match args.next() {
            Some(fg) => match fg.as_str() {
                "-i" => true,
                _ => false,
            },
            None => false,
        };

        Ok(Config {
            query,
            filepath,
            case_insensitive_var: env::var("IGNORE_CASE").is_ok(),
            case_insensitive_flag,
        })
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
