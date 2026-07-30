use crate::Config;
use std::fs;

pub fn find_match(config: Config) -> Result<Vec<(usize, String)>, ()> {
    match fs::read_to_string("message.txt") {
        Ok(value) => {
            let contents = value.as_str();
            let mut matches: Vec<(usize, String)> = vec![];
            for (number, line) in contents.lines().enumerate() {
                if line.contains(config.pattern) {
                    matches.push((number + 1, line.to_string()));
                }
            }
            Ok(matches)
        }

        Err(_error) => Err(()),
    }
}
