use std::iter;

pub fn build_proverb(words: &[&str]) -> String {
    if let Some(first) = words.first() {
        return words
            .windows(2)
            .map(phrase)
            .chain(iter::once(last_phrase(first)))
            .collect::<Vec<_>>()
            .join("\n");
    }

    String::new()
}

fn phrase(words: &[&str]) -> String {
    format!("For want of a {} the {} was lost.", words[0], words[1])
}

fn last_phrase(word: &str) -> String {
    format!("And all for the want of a {}.", word)
}
