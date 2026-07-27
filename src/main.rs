use std::env;
struct Config<'a> {
    pattern: &'a String,
    path: &'a String,
}
impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Config<'a> {
        Config {
            pattern: &args[1],
            path: &args[2],
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("{}\n {}", config.pattern, config.path);
}
