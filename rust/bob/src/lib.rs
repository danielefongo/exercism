pub fn reply(message: &str) -> &str {
    match message.trim() {
        "" => "Fine. Be that way!",
        m if is_all_capitals(m) && is_question(m) => "Calm down, I know what I'm doing!",
        m if is_question(m) => "Sure.",
        m if is_all_capitals(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_all_capitals(message: &str) -> bool {
    message == message.to_uppercase() && message.chars().any(|chr| chr.is_alphabetic())
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}
