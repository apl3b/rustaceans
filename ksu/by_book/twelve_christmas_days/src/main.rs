fn main() {
    let days_with_objects = [
        ("first", "A partridge in a pear tree."),
        ("second", "Two turtle doves,"),
        ("third", "Three French hens,"),
        ("fourth", "Four calling birds,"),
        ("fifth", "Five gold rings,"),
        ("sixth", "Six geese a-laying,"),
        ("seventh", "Seven swans a-swimming,"),
        ("eighth", "Eight maids a-milking,"),
        ("ninth", "Nine ladies dancing,"),
        ("tenth", "Ten lords a-leaping,"),
        ("eleventh", "Eleven pipers piping,"),
        ("twelfth", "Twelve drummers drumming,"),
    ];

    for i in 0..days_with_objects.len() {
        println!("On the {} day of Christmas my true love sent to me", days_with_objects[i].0);
        for j in (0..i+1).rev() {
            println!("{}", days_with_objects[j].1);
        }
        println!();
    }
}
