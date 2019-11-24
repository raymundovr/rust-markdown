#[derive(Debug)]
enum State {
    Normal,
    Uppercase,
    Lowercase,
    Comment,
}
use State::*;

fn machine_cycle(c: char, s: State) -> (Option<char>, State) {
    match (c, s) {
        ('^', Normal) => (None, Uppercase),
        ('_', Normal) => (None, Lowercase),
        ('#', Normal) => (None, Comment),
        ('^', Uppercase) => (None, Normal),
        ('_', Lowercase) => (None, Normal),
        ('#', Comment) => (None, Normal),
        (_, Comment) => (None, Comment),
        (other, Normal) => (Some(other), Normal),
        (other, Uppercase) => (Some(other.to_ascii_uppercase()), Uppercase),
        (other, Lowercase) => (Some(other.to_ascii_lowercase()), Lowercase),
    }
}

fn main() {
    let mut state = Normal;
    let text = String::from("This is _loWeRCAse_ then ^uppercase^ then a comment #comment here#");
    let mut processed_text = String::new();

    for character in text.chars() {
        let (output, new_state) = machine_cycle(character, state);
        if let Some(c) = output {
            processed_text.push(c);
        }

        state = new_state;
    }

    println!("{}", processed_text);
}
