use std::collections::HashMap;

fn main() {
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(1);
    numbers.push(3);
    numbers.push(17);
    numbers.push(-3);
    numbers.push(3);

    println!("Unsorted array: {:?}.", numbers);

    // Median
    numbers.sort();

    println!("Sorted array: {:?}.", numbers);
    let length = numbers.len();

    println!("Array length is {length}.");
    if length == 0 {
        return;
    }

    if let Some(median) = numbers.get(length / 2) {
        println!("The median is {median}");
    }

    // Mode
    let mut map = HashMap::new();
    let mut max = (0, 0);
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
        if *count > max.0 {
            max.0 = *count;
            max.1 = number;
        }
    }

    println!("The value that occurs more often: {}", max.1);
}
