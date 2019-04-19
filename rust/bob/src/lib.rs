enum Speech {
    Question,
    Yell,
    YellQuestion,
    Silence,
    Other,
}

pub fn reply(message: &str) -> &str {
    match classify(message) {
        Speech::Question => "Sure.",
        Speech::Yell => "Whoa, chill out!",
        Speech::YellQuestion => "Calm down, I know what I'm doing!",
        Speech::Silence => "Fine. Be that way!",
        Speech::Other => "Whatever.",
    }
}

fn classify(message: &str) -> Speech {
    let string = String::from(message.trim());

    if string.is_empty() {
        return Speech::Silence;
    } else if is_yelling(&string) && string.ends_with("?") {
        return Speech::YellQuestion;
    } else if string.ends_with("?") {
        return Speech::Question;
    } else if is_yelling(&string) {
        return Speech::Yell;
    }

    Speech::Other
}

fn is_yelling(message: &String) -> bool {
    message.to_uppercase().eq(message) && !message.to_lowercase().eq(message)
}
