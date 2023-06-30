use std::env;
use std::fs;
use std::process;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

enum SearchMode {
    File,     // Search for file or folder name
    Content,  // Search inside a specific file
}

struct Config {
    query: String,
    target: String,
    mode: SearchMode,
    depth: Option<usize>,
    num_threads: usize,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
    
        let query = args[1].clone();
        let target = args[2].clone();
        let mode = match args.get(3).map(|s| s.as_str()) {
            Some("file") => SearchMode::File,
            Some("content") => SearchMode::Content,
            _ => return Err("invalid search mode"),
        };
        let depth = args.get(4).and_then(|arg| arg.parse::<usize>().ok()).unwrap_or(0);
        let num_threads = args.get(5).and_then(|arg| arg.parse::<usize>().ok()).unwrap_or(2);
    
        Ok(Config { query, target, mode, depth: Some(depth), num_threads })
    }    
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
            search_file_or_folder(&config.query, &config.target, config.depth, config.num_threads);
        }
        SearchMode::Content => {
            search_inside_file(&config.query, &config.target);
        }
    }    
}

fn search_file_or_folder(query: &str, target: &str, depth: Option<usize>, num_threads: usize) {
    let (sender, receiver) = mpsc::channel();

    let target_path = Path::new(target);
    let depth = depth.unwrap_or(0);

    for _ in 0..num_threads {
        let sender_clone = sender.clone();
        let query_clone = query.to_owned();
        let target_path_clone = target_path.to_path_buf();
        let depth_clone = depth;
        thread::spawn(move || {
            search_recursive(&query_clone, &target_path_clone, depth_clone, 0, sender_clone);
        });
    }

    drop(sender);

    for result in receiver {
        if let Ok(path) = result {
            println!("{}", path.display());
        }
    }
}

fn search_recursive(query: &str, target: &PathBuf, depth: usize, current_depth: usize, sender: mpsc::Sender<Result<PathBuf, std::io::Error>>) {
    if current_depth > depth {
        return;
    }

    let entries = match fs::read_dir(target) {
        Ok(entries) => entries,
        Err(err) => {
            sender.send(Err(err)).unwrap();
            return;
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name.contains(query) {
                sender.send(Ok(path.clone())).unwrap();
            }

            if path.is_dir() {
                let sender_clone = sender.clone();
                let query_clone = query.to_owned();
                let path_clone = path.clone();
                let depth_clone = depth;
                let current_depth_clone = current_depth + 1;
                thread::spawn(move || {
                    search_recursive(&query_clone, &path_clone, depth_clone, current_depth_clone, sender_clone);
                });
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
