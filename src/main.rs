use std::env;
use std::fs;
use std::process;

enum SearchMode {
    File,   // Search for file or folder name
    Content, // Search inside a specific file
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for: {}", config.query);
    println!("in: {}", config.target);

    match config.mode {
        SearchMode::File => {
            // TODO: Implement search for file or folder name
        }
        SearchMode::Content => {
            // TODO: Implement search inside a specific file
        }
    }
}

struct Config {
    query: String,
    target: String,
    mode: SearchMode,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let target = args[2].clone();
        let mode = match args[3].as_str() {
            "file" => SearchMode::File,
            "content" => SearchMode::Content,
            _ => return Err("invalid search mode"),
        };

        Ok(Config { query, target, mode })
    }
}
