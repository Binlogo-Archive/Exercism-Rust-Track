pub fn reply(message: &str) -> &str {
    // unimplemented!("have Bob reply to the incoming message: {}", message)
    let trimed = message.trim();
    if trimed.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = trimed.ends_with("?");
    let is_yell = trimed.contains(char::is_alphabetic) && !trimed.contains(char::is_lowercase);
    match (is_question, is_yell) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
