use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];


    println!("in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("error in reading file!");
    
    println!("with text:\n{contents}");
}
