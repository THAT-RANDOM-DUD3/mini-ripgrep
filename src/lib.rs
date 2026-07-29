mod filesearch;

#[derive(Debug)]
pub struct Config<'a> {
    pub pattern: &'a String,
    pub path: &'a String,
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
                "Isufficient arguments passed to the program",
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
}
