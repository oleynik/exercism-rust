pub fn reply(message: &str) -> &str {
    match (
        message.trim().is_empty(),
        message.trim().ends_with("?"),
        message.trim() == message.trim().to_uppercase()
            && message.trim().chars().any(|c| c.is_alphabetic()),
    ) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, false) => "Sure.",
        (_, _, true) => "Whoa, chill out!",
        (_, _, _) => "Whatever.",
    }
}
