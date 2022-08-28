const SEPARATOR: &str = " ,:-_";

pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return "".to_string();
    }
    generate_abbreviate(phrase)
}

enum State {
    Begin,
    Separator,
    Lowercase,
    Uppercase,
}

impl State {
    fn transit(&mut self, ch: char) {
        if is_separator(ch) {
            *self = State::Separator;
        } else if ch.is_ascii_uppercase() {
            *self = State::Uppercase;
        } else {
            *self = State::Lowercase;
        }
    }
}

fn generate_abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut state = State::Begin;

    for ch in phrase.chars() {
        match state {
            State::Begin => acronym.push(ch.to_ascii_uppercase()),
            State::Separator => {
                if !is_separator(ch) {
                    acronym.push(ch.to_ascii_uppercase());
                }
            }
            State::Lowercase => {
                if ch.is_uppercase() {
                    acronym.push(ch);
                }
            }
            State::Uppercase => (),
        }
        state.transit(ch);
    }

    acronym
}

fn is_separator(ch: char) -> bool {
    SEPARATOR.contains(ch)
}
