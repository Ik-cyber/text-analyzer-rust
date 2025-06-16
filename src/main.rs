mod analysis;
mod input;

use analysis::{count_characters, count_words, search_word};
use input::{read_from_file, read_from_terminal};

use std::io::{self};

fn main() {
    println!("How would you like to input text?");
    println!("1. Type text directly");
    println!("2. Provide file path");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice = choice.trim();

    let input_text = match choice {
        "1" => read_from_terminal(),
        "2" => read_from_file(),
        _ => {
            println!("Invalid choice. Exiting.");
            return;
        }
    };

    println!("\nYour input:\n{}", input_text);

    let (total_chars, total_non_space_chars) = count_characters(&input_text);
    println!(
        "Total Characters: {}, Total Non-Space Characters: {}",
        total_chars, total_non_space_chars
    );

    let word_count = count_words(&input_text);
    println!("Word Count: {}", word_count);

    // ✅ New part for searching
    println!("Do you want to search for a word? (y/n)");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input");
    let answer = answer.trim();

    if answer == "y" {
        println!("Enter the word you want to search for:");
        let mut search_word_input = String::new();
        io::stdin()
            .read_line(&mut search_word_input)
            .expect("Failed to read input");
        let search_word_input = search_word_input.trim();

        let (count, positions) = search_word(&input_text, search_word_input);
        println!("The word '{}' appeared {} times.", search_word_input, count);
        println!("Positions: {:?}", positions);
    }
}
