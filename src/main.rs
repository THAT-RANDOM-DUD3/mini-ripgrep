use ripgrep::Config;
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
    println!("{}\n{}", config.pattern, config.path);
}
