pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic())
        .map(letters_in_word)
        .collect::<String>()
}

fn letters_in_word(word: &str) -> String {
    if word.is_empty() {
        String::from("")
    } else if word.chars().all(char::is_lowercase) || word.chars().all(char::is_uppercase) {
        String::from(&word[..1]).to_uppercase()
    } else {
        // camel case, return each upper
        word.chars()
            .filter(|c| c.is_uppercase())
            .collect::<String>()
    }
}
