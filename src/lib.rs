pub mod filesearch;

#[derive(Debug)]
pub struct Config<'a> {
    pattern: &'a str,
    path: &'a str,
}
#[derive(Debug)]
pub enum Error {
    InsufficientArguments(String),
    TooMuchArguments(String),
}
impl<'a> Config<'a> {
    fn validate_argument(args: &'a [String]) -> Result<Config<'a>, Error> {
        if args.len() == 3 {
            Ok(Config {
                pattern: &args[1],
                path: &args[2],
            })
        } else if args.len() < 3 {
            Err(Error::InsufficientArguments(String::from(
                "Insufficient arguments passed to the program",
            )))
        } else {
            Err(Error::TooMuchArguments(String::from(
                "Too many arguments passed to the program",
            )))
        }
    }
    pub fn new(args: &'a [String]) -> Result<Config<'a>, Error> {
        Self::validate_argument(args)
    }

    pub fn pattern(&self) -> &'a str {
        self.pattern
    }

    pub fn path(&self) -> &'a str {
        self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_few_arguments_returns_error() {
        let args = vec![String::from("program")];
        let result = Config::new(&args);
        assert!(matches!(result, Err(Error::InsufficientArguments(_))));
    }

    #[test]
    fn valid_arguments_creates_config() {
        let args = vec![
            String::from("program"),
            String::from("pattern"),
            String::from("path.txt"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.pattern(), "pattern");
        assert_eq!(config.path(), "path.txt");
    }
}
