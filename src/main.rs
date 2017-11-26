extern crate rustgrep;

use std::env;
use std::process;

use rustgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = rustgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}