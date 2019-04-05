use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1)
    });

    println!("query {}", config.query);
    println!("filename {}", config.filename);

    let _contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    //println!("contents {}", contents);
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1][..];
        let filename = &args[2][..];
        Ok(Config { query, filename })
    }
}
