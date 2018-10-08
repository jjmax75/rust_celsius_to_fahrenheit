use std::io;

fn main() {
    println!("What temperature in celsius would you like to convert to fahrenheit?");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: i32 = temperature.trim().parse().expect("invalid input");

    println!("{} degress Celsius is {:?} degrees Fahrenheit", temperature, celsius_to_fahrenheit(temperature));
}

fn celsius_to_fahrenheit(temperature: i32) -> i32 {
    (temperature * 9/5) + 32
}
