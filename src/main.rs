use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config{
    query: String,
    filepath: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });;

    // let (query, filepath) = parse_config(&args);

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filepath);
    println!(" ");

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)
        .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}