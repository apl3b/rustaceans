
use std::io;

fn main() {
    let mut x = 2.0;
    let y: f32 = 3.0;

    println!("x: {x}");
    println!("y: {y}");

    x += 2.0;
    println!("new x: {x}");

    let t = true;
    println!("boolean value: {t}");

    let t: bool = false;
    println!("new boolean value: {t}");

    let heart_eyed_cat = '😻';
    println!("cat eyes: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("the y from tup is {y}");

    let third = tup.2;
    println!("The third element of the tuple is {third}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];
    println!("The element of index {index} of array is {element}");
}
