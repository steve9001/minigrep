use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query {}", config.query);
    println!("filename {}", config.filename);

    if let Err(e) = run(config) {
        println!("Error running search: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;
        //.unwrap_or_else(|err| {
        //return Err("Something went wrong reading the file");
        //return "Something went wrong reading the file";
    //});
        //.expect("Something went wrong reading the file");
    //println!("contents {}", contents);
    Ok(())
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
