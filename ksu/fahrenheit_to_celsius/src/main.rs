use std::io;

fn main() {
    println!("Enter Fahrenheit temperature you want to convert to Celsius.");

    let mut f_temperature = String::new();

    io::stdin()
        .read_line(&mut f_temperature)
        .expect("Error while reading input.");

    let f_temperature: f64 = f_temperature.trim().parse().expect("Number is expected.");

    let c_temperature: f64 = (f_temperature - 32.0) * 5.0 / 9.0;
    println!("The Celsius temperature of {f_temperature} Fahrenheit is {c_temperature}.");
}
