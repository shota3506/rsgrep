use std::error::Error;
use std::io;
use std::io::prelude::*;

use clap::{App, Arg};
use regex;

pub struct Config {
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        const PATTERN_KEY: &str = "PATTERN";
        const CASE_SENSITIVE_KEY: &str = "case-sensitive";

        let matches = App::new("rsgrep")
            .about("Simple grep command")
            .arg(
                Arg::with_name(PATTERN_KEY)
                    .required(true)
                    .help("A regular expression used for searching"),
            )
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
    let query = config.query;
    let re = regex::RegexBuilder::new(&query)
        .case_insensitive(!config.case_sensitive)
        .build()?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let m = re.find(&line);
        if let Some(_) = m {
            write(&mut io::stdout().lock(), &line)?;
        }
    }

    Ok(())
}

fn write(w: &mut dyn io::Write, result: &str) -> Result<(), Box<dyn Error>> {
    let mut s = result.to_string();
    s.push_str("\n");
    w.write(s.as_bytes())?;
    Ok(())
}
