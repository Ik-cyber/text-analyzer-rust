pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn count_characters(text: &str) -> (usize, usize) {
    let total_chars = text.len();
    let non_space_chars = text.chars().filter(|c| !c.is_whitespace()).count();
    (total_chars, non_space_chars)
}

// ✅ New function for searching a word
pub fn search_word(text: &str, word: &str) -> (usize, Vec<usize>) {
    let mut count = 0;
    let mut positions = Vec::new();

    // Convert text to lowercase to make it case-insensitive (optional)
    let lower_text = text.to_lowercase();
    let lower_word = word.to_lowercase();

    let words: Vec<&str> = lower_text.split_whitespace().collect();

    for (index, w) in words.iter().enumerate() {
        if *w == lower_word {
            count += 1;
            positions.push(index);
        }
    }

    (count, positions)
}

