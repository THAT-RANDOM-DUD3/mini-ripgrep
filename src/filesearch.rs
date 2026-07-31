use crate::Config;
use std::fs;

pub fn find_match(contents: &str, pattern: &str) -> Vec<(usize, String)> {
    let mut matches: Vec<(usize, String)> = vec![];
    for (number, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            matches.push((number + 1, line.to_string()));
        }
    }
    matches
}

pub fn search_file(config: &Config) -> Result<Vec<(usize, String)>, std::io::Error> {
    let contents = fs::read_to_string(config.path())?;
    Ok(find_match(&contents, config.pattern()))
}
