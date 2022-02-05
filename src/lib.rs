use std::error::Error;
use std::io::{self, Read};

use clap::{App, Arg};

mod search;

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
        search::search(&config.query, &contents)?
    } else {
        search::search_case_insensitive(&config.query, &contents)?
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
