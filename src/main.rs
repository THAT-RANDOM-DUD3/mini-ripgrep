use ripgrep::Config;
use ripgrep::filesearch;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| match error {
        ripgrep::Error::InsufficientArguments(value) => {
            println!("{value}");
            process::exit(1);
        }

        ripgrep::Error::TooMuchArguments(value) => {
            println!("{value}");
            process::exit(1);
        }
    });

    if let Ok(result) = filesearch::search_file(config) {
        for (line_number, line_content) in result {
            println!("Line {} {}", line_number, line_content);
        }
    }
}
