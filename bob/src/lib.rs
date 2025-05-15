pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if is_yelling_question(trimmed) {
        "Calm down, I know what I'm doing!"
    } else if is_question(trimmed) {
        "Sure."
    } else if is_yelling(trimmed) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    let has_letters = message.chars().any(|c| c.is_ascii_alphabetic());
    has_letters && message.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_uppercase())
}

fn is_yelling_question(message: &str) -> bool {
    is_question(message) && is_yelling(message)
}

