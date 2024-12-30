use std::io;

fn main() {
    println!("How many Fibonacci numbers you want to generate?");

    let mut count = String::new();
    io::stdin()
        .read_line(&mut count)
        .expect("Error while reading input.");

    let mut count: u32 = count.trim().parse().expect("Expect number as an input!");
    let mut prev = 0;
    let mut current = 1;
    println!("-------");
    while count > 0 {
        println!("{current}");
        let temp = prev;
        prev = current;
        current += temp;
        count -= 1;
    }
}
