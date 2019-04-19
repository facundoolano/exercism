pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 1;
    let mut current = '/';

    for slice in source.chars().collect::<Vec<_>>().windows(2) {
        let previous = slice[0];
        current = slice[1];
        if current == previous {
            count += 1;
        } else {
            result = new_result(result, previous, count);
            count = 1;
        }
    }

    new_result(result, current, count)
}

fn new_result(result: String, c: char, count: u32) -> String {
    if count <= 1 {
        format!("{}{}", result, c)
    } else {
        format!("{}{}{}", result, count, c)
    }
}


pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut digit_accumulator = String::new();

    for c in source.chars() {
        if c.is_numeric() {
            digit_accumulator.push(c);
        } else {
            if let Ok(digit) = digit_accumulator.parse() {
                result = format!("{}{}", result, c.to_string().repeat(digit));
            } else {
                result.push(c);
            }
            digit_accumulator = String::new();
        }
    }

    result
}
