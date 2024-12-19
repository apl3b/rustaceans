use std::io;

fn main() {
    println!("Give me a word and I will convert it to pig latin!");
    println!("Type QUIT to finish.");
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    loop {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Error while reading input.");
        let word = String::from(word.trim());

        if word == "QUIT" {
            println!("Bye!");
            break;
        }

        if word.len() == 1 {
            println!("Enter a bigger word!");
            continue;
        }

        if let Some(first) = word.chars().next() {
            if !first.is_alphabetic() {
                println!("Should start with a letter!");
                continue;
            }

            if vowels.contains(&first) {
                let pig_version = word + "-hay";
                println!("The pig version of the word is {pig_version}.");
            } else {
                let mut without_first = word.chars();
                without_first.next();
                let pig_version =
                    String::from(without_first.as_str()) + "-" + &first.to_string() + "ay";
                println!("The pig version of the word is {}.", pig_version);
            }
        }
    }
}
