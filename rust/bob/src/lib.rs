pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let is_empty = message.is_empty();
    let is_shout = message.to_uppercase() == message && message.to_lowercase() != message;
    let is_question = message.ends_with('?');

    match (is_empty, is_shout, is_question) {
        (true, _, _) => "Fine. Be that way!",
        (false, false, true) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (false, true, true) => "Calm down, I know what I'm doing!",
        (false, false, false) => "Whatever.",
    }
}
