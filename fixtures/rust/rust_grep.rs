use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pattern> <file>", args.get(0).map(|s| s.as_str()).unwrap_or("rust_grep"));
        return;
    }

    let pattern = &args[1];
    let file_path = &args[2];

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", file_path, e);
            return;
        }
    };

    let reader = BufReader::new(file);
    for (line_num, line) in reader.lines().enumerate() {
        if let Ok(line_str) = line {
            if line_str.contains(pattern) {
                println!("{}:{}:{}", file_path, line_num + 1, line_str);
            }
        }
    }
}
