fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; 
    println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The values of x in inner scope is {x}");
    }

    println!("The values of x in outer scope is {x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
}
            