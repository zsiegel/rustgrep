use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    line_number: usize,
    contents: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //skip over the executable name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string found"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename found"),
        };

        return Ok(Config { query, filename });
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let filename = &config.filename;
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    for search_result in search(&config.query, &mut reader) {
        println!(
            "{}:{} {}",
            filename,
            search_result.line_number,
            search_result.contents
        );
    }
    Ok(())
}

pub fn search<'a>(query: &str, reader: &mut BufReader<File>) -> Vec<SearchResult> {
    reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)|
            match line {
                Err(_) => None,
                Ok(content) =>
                    if content.contains(query) {
                        Some(SearchResult {
                            line_number: idx + 1,
                            contents: content.trim_left().to_string(),
                        })
                    } else {
                        None
                    }
            })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive\nPick Three.";

        assert_eq!(
            vec![
                SearchResult {
                    line_number: 2,
                    contents: String::from("safe, fast, productive"),
                },
            ],
            search(query, contents)
        );
    }
}
