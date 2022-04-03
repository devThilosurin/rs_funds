fn reverses_str(word: &str) -> String {
    if word.len() == 0 {
        return String::new();
    }

    // let mut chars = word.to_string();
    // let last = chars.pop().unwrap();
    // format!("{}{}", last, reverses_str(chars.as_str()))

    // let ws = word.split_at(word.len() - 1);
    // let last = ws.1.to_string();
    // format!("{}{}", last, reverses_str(ws.0))

    let mut chars = word.chars();
    let last = chars.next_back().unwrap().to_string();
    format!("{}{}", last, reverses_str(chars.as_str()))
}

fn main() {
    println!(" -> {:?}", reverses_str("Hello World!"));
}

fn solution_short(phrase: &str) -> String {
    phrase.chars().rev().collect()
}
