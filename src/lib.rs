use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    line_number: usize,
    contents: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //skip over the executable name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string found")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename found")
        };

        return Ok(Config {
            query,
            filename
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let filename = &config.filename;
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for search_result in search(&config.query, &contents) {
        println!("{}:{} {}", filename, search_result.line_number, search_result.contents);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<SearchResult> {
    contents.lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.contains(query) {
                Some(SearchResult {
                    line_number: idx + 1,
                    contents: line.trim_left().to_string()
                })
            } else {
                None
            }
        }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive\nPick Three.";

        assert_eq!(vec![SearchResult { line_number: 2, contents: String::from("safe, fast, productive") }],
                   search(query, contents));
    }
}