use std::error::Error;
use std::io;

use clap::{App, Arg};

mod search;

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
    let contents = read(&mut io::stdin().lock())?;

    let results = if config.case_sensitive {
        search::search(&config.query, &contents)?
    } else {
        search::search_case_insensitive(&config.query, &contents)?
    };

    write(&mut io::stdout().lock(), results)?;

    Ok(())
}

fn read(r: &mut dyn io::Read) -> Result<String, io::Error> {
    let mut contents = String::new();
    r.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write(w: &mut dyn io::Write, results: Vec<&str>) -> Result<(), Box<dyn Error>> {
    for line in results {
        let mut s = line.to_string();
        s.push_str("\n");
        w.write(s.as_bytes())?;
    }
    Ok(())
}
