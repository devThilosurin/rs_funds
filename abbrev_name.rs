fn abbrev_name(name: &str) -> String {
    let coll_str: Vec<String> = name
        .split(" ")
        .map(|s| s.to_ascii_uppercase().as_bytes()[0] as char)
        .map(|c| c.encode_utf8(&mut [0u8, 1]).to_string())
        .collect::<Vec<_>>()
        .join(".");

    coll_str.join(".")
}

fn main() {
    println!(" -> {:?}", abbrev_name("Sam Harris"));
    println!(" -> {:?}", abbrev_name("Patrick Feenan"));
    println!(" -> {:?}", abbrev_name("Evan Cole"));
    println!(" -> {:?}", abbrev_name("P Favuzzi"));
    println!(" -> {:?}", abbrev_name("David Mendieta"));
}

fn solution_short(name: &str) -> String {
    name.split_whitespace()
        .map(|word| word[0..1].to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
}
