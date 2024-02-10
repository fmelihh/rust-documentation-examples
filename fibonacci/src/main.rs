use std::io;

fn main() {
    loop {
        println!("Input your nth fibonacci number");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("failed to read line!");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("provided nth number is invalid.");
                continue;
            }
        };

        let nth_number = generate_fibonacci(number);
        println!("The nth number is {nth_number}");
        break;
    }
}

fn generate_fibonacci(n: u32) -> u32 {
    let mut first: u32 = 1;
    let mut second: u32 = 1;

    for _ in 1..(n + 1) {
        println!("first: {first}, second: {second}");
        (first, second) = (second, second + first)
    }
    second
}
