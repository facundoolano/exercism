pub fn brackets_are_balanced(string: &str) -> bool {
    let mut pairs = Vec::<char>::new();

    for c in string.chars() {
        match (pairs.last(), c) {
            (Some('{'), '}') | (Some('['), ']') | (Some('('), ')') => {
                pairs.pop();
            }
            (_, _) if is_bracket(c) => {
                pairs.push(c);
            }
            _ => {}
        }
    }

    pairs.is_empty()
}

fn is_bracket(c: char) -> bool {
    ['{', '}', '[', ']', '(', ')'].contains(&c)
}
