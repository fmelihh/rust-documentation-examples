// CHAPTER 3 HOMEWORK

//PROBLEM DESCRIPTION
// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    loop {
        println!("Please input your temperature!");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line!");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Provided temperature value is invalid. number {temperature}");
                continue;
            }
        };

        println!("Please input your current temperature kind.");
        let mut symbol_to_convert = ' ';
        let mut current_symbol = String::new();
        io::stdin()
            .read_line(&mut current_symbol)
            .expect("Failed to read line!");

        let current_symbol: char = match current_symbol.trim().parse() {
            Ok(symbol) => {
                if symbol == 'F' {
                    symbol_to_convert = 'C';
                } else if symbol == 'C' {
                    symbol_to_convert = 'F';
                } else {
                    panic!("Provided kind is invalid. Kind must be F or C.")
                }

                symbol
            }
            Err(_) => {
                println!("Provided kind is invalid. kind {current_symbol}");
                continue;
            }
        };

        println!("Symbol: {current_symbol}, Temperature: {temperature}");

        if current_symbol == 'F' {
            let c = convert_f_to_c(temperature);
            println!("C: {c}");
        } else {
            let f = convert_c_to_f(temperature);
            println!("F: {f}");
        }
    }
}

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) / (1.8000)
}

fn convert_c_to_f(c: f64) -> f64 {
    (c * 1.8) + 32.0
}
