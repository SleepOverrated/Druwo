use std::{fs, process};

fn compile(src_file: &str) {
    let content = match fs::read_to_string(src_file) {
        Ok(result) => result,
        _ => {
            eprintln!("Failed to read file: {}", src_file);
            process::exit(1);
        },
    };
    let words: Vec<&str> = content.split_whitespace().collect();

    for i in 0..words.len() {
        println!("Word {}: {}", i, words[i]);
    }
}
