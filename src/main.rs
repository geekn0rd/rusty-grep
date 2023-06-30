use std::env;
use std::fs;
use std::process;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

enum SearchMode {
    File,     // Search for file or folder name
    Content,  // Search inside a specific file
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
            search_file_or_folder(&config.query, &config.target, config.depth);
        }
        SearchMode::Content => {
            search_inside_file(&config.query, &config.target);
        }
    }    
}

struct Config {
    query: String,
    target: String,
    mode: SearchMode,
    depth: Option<usize>,
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
        let depth = args.get(4).and_then(|arg| arg.parse::<usize>().ok()).unwrap_or(0);

        Ok(Config { query, target, mode, depth: Some(depth) })
    }
}

fn search_file_or_folder(query: &str, target: &str, depth: Option<usize>) {
    search_recursive(query, &PathBuf::from(target), depth, 0);
}

fn search_recursive(query: &str, target: &PathBuf, depth: Option<usize>, current_depth: usize) {
    if let Some(max_depth) = depth {
        if current_depth > max_depth {
            return;
        }
    }

    let entries = fs::read_dir(target).unwrap();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name.contains(query) {
                println!("{}", path.display());
            }

            if path.is_dir() {
                search_recursive(query, &path, depth, current_depth + 1);
            }
        }
    }
}

fn search_inside_file(query: &str, target: &str) {
    let file = fs::File::open(target).unwrap();
    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.contains(query) {
                println!("Line {}: {}", line_number + 1, line);
            }
        }
    }
}
