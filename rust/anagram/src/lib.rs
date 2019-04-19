use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut possible_anagrams: HashSet<&str> = possible_anagrams.iter().cloned().collect();
    possible_anagrams.retain(|&anagram| is_anagram(word, anagram));
    possible_anagrams
}

fn is_anagram(word: &str, anagram: &str) -> bool {
    anagram.to_lowercase() != word.to_lowercase() && word_count(anagram) == word_count(word)
}

fn word_count(word: &str) -> HashMap<char, u32> {
    let mut count_map = HashMap::new();
    for c in word.to_lowercase().chars() {
        let entry = count_map.entry(c).or_insert(0);
        *entry += 1;
    }

    count_map
}
