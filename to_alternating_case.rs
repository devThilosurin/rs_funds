fn to_alternating_case(s: &str) -> String {
    let collect_str: Vec<String> = s
        .chars()
        .map(|x| -> String {
            if x.is_ascii_lowercase() {
                x.to_ascii_uppercase().to_string()
            } else {
                x.to_ascii_lowercase().to_string()
            }
        })
        .collect();

    collect_str.join("")
}

fn main() {
    println!(" -> {:?}", to_alternating_case("hello world"));
    println!(" -> {:?}", to_alternating_case("HELLO WORLD"));
    println!(" -> {:?}", to_alternating_case("hello WORLD"));
    println!(" -> {:?}", to_alternating_case("HeLLo WoRLD"));
    println!(
        " -> {:?}",
        to_alternating_case(&to_alternating_case("Hello World")[..])
    );
}
