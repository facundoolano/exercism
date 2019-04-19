pub fn verse(n: i32) -> String {
    let v1 = format!(
        "{} on the wall, {}.",
        capfirst(pluralize_bottle(n)),
        pluralize_bottle(n)
    );
    let v2 = if n > 1 {
        format!(
            "Take one down and pass it around, {} on the wall.",
            pluralize_bottle(n - 1)
        )
    } else if n == 1 {
        String::from("Take it down and pass it around, no more bottles of beer on the wall.")
    } else {
        String::from("Go to the store and buy some more, 99 bottles of beer on the wall.")
    };
    format!("{}\n{}\n", v1, v2)
}

pub fn sing(start: i32, end: i32) -> String {
    (end..(start + 1))
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}

fn pluralize_bottle(n: i32) -> String {
    match n {
        0 => String::from("no more bottles of beer"),
        1 => String::from("1 bottle of beer"),
        n => format!("{} bottles of beer", n),
    }
}

fn capfirst(mut s: String) -> String {
    s.get_mut(..1).map(|c| {
        c.make_ascii_uppercase();
        &*c
    });
    s
}
