use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("searching for {}", config.query);
    println!("in file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("error in reading file!");
    
    println!("with text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}