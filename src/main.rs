use std::env;
use std::fs;
use std::process;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

// Define an enumeration for the search mode
enum SearchMode {
    File,     // Search for file or folder name
    Content,  // Search inside a specific file
}

// Define a struct to hold the configuration options
struct Config {
    query: String,
    target: String,
    mode: SearchMode,
    depth: Option<usize>,
    num_threads: usize,
    invert_match: bool, // Added field for the "anti-grep" behavior
}

impl Config {
    // Build the configuration from command-line arguments
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
    
        // Determine the search mode based on the argument provided
        let mode = match args.get(1).map(|s| s.as_str()) {
            Some("file") => SearchMode::File,
            Some("content") => SearchMode::Content,
            _ => return Err("invalid search mode"),
        };
        
        // Extract the query, target, depth, and num_threads arguments
        let query = args[2].clone();
        let target = args[3].clone();
        let depth = args.get(4).and_then(|arg| arg.parse::<usize>().ok()).unwrap_or(0);
        let num_threads = args.get(5).and_then(|arg| arg.parse::<usize>().ok()).unwrap_or(2);
    
        // Check if the --invert or --no-match flag is present
        let invert_match = args.iter().any(|arg| arg == "--invert");
    
        Ok(Config { query, target, mode, depth: Some(depth), num_threads, invert_match })
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
            search_inside_file(&config.query, &config.target, config.invert_match);
        }
    }    
}

// Search for files or folders that match the query
fn search_file_or_folder(query: &str, target: &str, depth: Option<usize>, num_threads: usize) {
    let (sender, receiver) = mpsc::channel();

    let target_path = Path::new(target);
    let depth = depth.unwrap_or(0);

    // Spawn multiple threads to perform parallel searching
    for _ in 0..num_threads {
        let sender_clone = sender.clone();
        let query_clone = query.to_owned();
        let target_path_clone = target_path.to_path_buf();
        let depth_clone = depth;
        thread::spawn(move || {
            search_recursive(&query_clone, &target_path_clone, depth_clone, 0, sender_clone, false);
        });
    }

    drop(sender);

    // Collect and display the search results
    for result in receiver {
        if let Ok(path) = result {
            println!("{}", path.display());
        }
    }
}

// Recursively search for files or folders that match the query
fn search_recursive(query: &str, target: &PathBuf, depth: usize, current_depth: usize, sender: mpsc::Sender<Result<PathBuf, std::io::Error>>, invert_match: bool) {
    if current_depth > depth {
        return;
    }

    // Read the directory entries
    let entries = match fs::read_dir(target) {
        Ok(entries) => entries,
        Err(err) => {
            sender.send(Err(err)).unwrap();
            return;
        }
    };

    // Iterate over each directory entry
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            let matches_query = name.contains(query);
            
            // Send the path if it matches the query and the invert_match flag is false,
            // or if it does not match the query and the invert_match flag is true
            if (matches_query && !invert_match) || (!matches_query && invert_match) {
                sender.send(Ok(path.clone())).unwrap();
            }

            // Recursively search inside subdirectories
            if path.is_dir() {
                let sender_clone = sender.clone();
                let query_clone = query.to_owned();
                let path_clone = path.clone();
                let depth_clone = depth;
                let current_depth_clone = current_depth + 1;
                thread::spawn(move || {
                    search_recursive(&query_clone, &path_clone, depth_clone, current_depth_clone, sender_clone, invert_match);
                });
            }
        }
    }
}

// Search for the query inside a specific file
fn search_inside_file(query: &str, target: &str, invert_match: bool) {
    let file = fs::File::open(target).unwrap();
    let reader = BufReader::new(file);

    // Read each line of the file and check for matches with the query
    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let matches_query = line.contains(query);
            
            // Display the line if it matches the query and the invert_match flag is false,
            // or if it does not match the query and the invert_match flag is true
            if (matches_query && !invert_match) || (!matches_query && invert_match) {
                println!("Line {}: {}", line_number + 1, line);
            }
        }
    }
}
