use std::fs;
use std::io;

pub fn read_from_terminal() -> String {
    println!("Enter your text:");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read text");
    text.trim().to_string()
}

pub fn read_from_file() -> String {
    println!("Enter the file path:");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read path");

    let path = path.trim();

    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Failed to read file. Exiting.");
            std::process::exit(1);
        }
    }
}
