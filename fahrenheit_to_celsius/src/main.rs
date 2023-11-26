use std::fmt::format;
use std::io;

fn main() {
    println!("Please select the conversion type: ");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let conversion_type: i32 = loop {
        let mut conversion_type: String = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");

        match conversion_type.trim() {
            "1" => break 1,
            "2" => break 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };
    };

    println!("Please input the temperature:");
    let temperature: f64 = loop {
        let mut temperature: String = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        match temperature.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please input a valid temperature");
                continue;
            }
        };
    };

    let converted_temperature = if conversion_type == 1 {
        (temperature - 32.) / 1.8
    } else {
        temperature * 1.8 + 32.
    };

    let result: String = if conversion_type == 1 {
        format!("{temperature} Fahrenheit to Celsius is: {converted_temperature}")
    } else {
        format!("{temperature} Celsius to Fahrenheit is: {converted_temperature}")
    };

    println!("{}", result);
}
