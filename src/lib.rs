use std::error::Error;
use std::io::{self, Read};

use clap::{App, Arg};

pub struct Config {
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        const PATTERN_KEY: &str = "<PATTERN>";
        const CASE_SENSITIVE_KEY: &str = "case-sensitive";

        let matches = App::new("rsgrep")
            .about("simple grep command")
            .arg(Arg::with_name(PATTERN_KEY).required(true))
            .arg(
                Arg::with_name(CASE_SENSITIVE_KEY)
                    .help("Search case sensitively")
                    .short("s")
                    .long("case-sensitive"),
            )
            .get_matches();

        let q = match matches.value_of(PATTERN_KEY) {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let case_sensitive = matches.is_present(CASE_SENSITIVE_KEY);

        Ok(Config {
            query: q.to_string(),
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read()?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn read() -> Result<String, io::Error> {
    let mut contents = String::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
