use std::fs;
use std::fmt;
use std::cmp::PartialEq;

type Line = u64;

static USAGE_STR: &str = "\
Usage:
minigrep [-i] <QUERY> <FILE>";

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

#[derive(Debug)]
pub struct Match<'a> {
    pub line: Line,
    pub text: &'a str,
}

impl<'a> Config<'a> {
    pub fn parse(args: &'a [String]) -> Result<Config,String> {
        match args.len() {
            3...4 => {
                if args.len() == 3 {
                    Ok(Config { query: &args[1], filename: &args[2], case_sensitive: true })
                } else if &args[1] == "-i" {
                    Ok(Config { query: &args[2], filename: &args[3], case_sensitive: false })
                } else {
                    Err(format!("First argument '{}' is not a valid option\n{}", &args[1], USAGE_STR))
                }
            },
            n if n < 3 => Err(format!("Not enough arguments\n{}", USAGE_STR)),
            _ => Err(format!("Too many arguments\n{}", USAGE_STR)),
        }
    }
}

impl<'a> Match<'a> {
    pub fn new(line: Line, text: &str) -> Match {
        Match { line, text }
    }
}

impl<'a> PartialEq for Match<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.text == other.text
    }
}

impl<'a> fmt::Display for Match<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

pub fn read_file(filename: &str) -> Result<String,String> {
    match fs::read_to_string(filename) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("Error reading \"{}\": {}", filename, e)),
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<Match<'a>> {
    let mut results = Vec::new();
    let mut line: Line = 1;
    for text in contents.lines() {
        if text.contains(query) {
            results.push(Match::new(line, text));
        }
        line += 1;
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Match<'a>> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut line: Line = 1;
    for text in contents.lines() {
        if text.to_lowercase().contains(&query_lower) {
            results.push(Match::new(line, text));
        }
        line += 1;
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<Match<'a>> {
    if case_sensitive {
        search_case_sensitive(query, contents)
    } else {
        search_case_insensitive(query, contents)
    }
}

pub fn run(args: Vec<String>) -> Result<(),String> {
    let config = Config::parse(&args)?;
    let contents = read_file(config.filename)?;
    let matches = search(config.query, &contents, config.case_sensitive);
    for line in matches {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![
                Match { line: 2, text: "safe, fast, productive." }
            ],
            search_case_sensitive(query, contents)
        );
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
            vec![
                Match { line: 1, text: "Rust:" },
                Match { line: 4, text: "Trust me." }
            ],
            search_case_insensitive(query, contents)
        );
    }
}
