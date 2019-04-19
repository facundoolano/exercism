use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let sentence_char_count = sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|&c| is_ascii_letter(c))
        .collect::<HashSet<char>>()
        .len();

    let ascii_char_count = (b'a'..=b'z').len();
    sentence_char_count == ascii_char_count
}

fn is_ascii_letter(c: char) -> bool {
    'a' <= c && c <= 'z'
}
