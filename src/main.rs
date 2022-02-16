extern crate rsgrep;

use std::process;

use rsgrep::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = rsgrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
