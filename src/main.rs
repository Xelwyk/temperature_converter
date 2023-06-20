use std::io;

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    return (celsius * 9.0/5.0) + 32.0
}
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * 5.0/9.0
}

fn main() {
    let mut input = String::new();
    println!("Input temperature");
    let temperature = match io::stdin()
            .read_line(&mut input) {
        Ok(_) => input.trim(),
        Err(_) => "-999",
    };

    let temperature = match temperature.parse::<f32>() {
        Ok(num) => num,
        Err(_) => -888.0,
    };
    println!("converted to celsius: {}", fahrenheit_to_celsius(temperature));
    println!("converted to fahrenheit: {}", celsius_to_fahrenheit(temperature));
}
