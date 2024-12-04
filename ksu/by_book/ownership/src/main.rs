fn main() {
    let input = String::from("Hello, world");
    let first_word = first_word(&input);
    println!("{first_word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
