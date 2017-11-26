use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

#[derive(Debug)]
pub struct SearchResult {
    line_number: usize,
    contents: String
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &SearchResult) -> bool {
        self.contents == other.contents && self.line_number == other.line_number
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments - pass a query string followed by a filename");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config {
            query,
            filename
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for search_result in search(&config.query, &contents) {
        println!("line {}: {}", search_result.line_number, search_result.contents);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    for (idx, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push(SearchResult {
                line_number: idx + 1,
                contents: line.trim_left().to_string()
            });
        }
    }
    results
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