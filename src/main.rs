use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

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

//fn parse_config(args: &Vec<String>) -> (&str, &str) {
fn parse_config(args: &[String]) -> Config {
    //let q = args[1].to_string();
    let query = &args[1][..];
    //let f = args[2].to_string();
    let filename = &args[2][..];
    //Config { query: &args[1], filename: &args[2] }
    //Config { query: q, filename: f }
    Config { query, filename }
}
