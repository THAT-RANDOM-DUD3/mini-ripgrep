use crate::Config;
use std::fs;

pub fn find_match<'a>(contents: &'a str, pattern: &'a str) -> Vec<(usize, String)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_number, line)| line.contains(pattern))
        .map(|(number, line)| (number + 1, line.to_string()))
        .collect::<_>()
}

pub fn search_file<'a>(config: Config<'a>) -> Result<Vec<(usize, String)>, std::io::Error> {
    let contents = fs::read_to_string(config.path())?;
    Ok(find_match(&contents, config.pattern()))
}
