pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        // not sure what they mean by zero length
        return vec![String::new(); digits.len() + 1];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| w.iter().collect())
        .collect()
}
