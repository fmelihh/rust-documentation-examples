use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");

    //integer types
    // un -> (0, 2^n - 1)
    // in (-2^n-1, 2^n-1)

    let num2: u8 = 1;
    let num1: i8 = -1;
    println!("{num1}, {num2}");

    // integer overflow
    // let num1: u8 = 258;
    // println!("{num1}")

    // floating point

    // Rust also has two primitive types for floating-point numbers,
    // which are numbers with decimal points. Rust’s floating-point types are f32
    // and f64, which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs, it’s roughly the same speed
    // as f32 but is capable of more precision. All floating-point types are signed.

    let x = 2.0;
    let y: f32 = 1.2;

    //numeric operations
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //boolean type
    let t = true;
    let f: bool = false;

    //character type (single quote)
    let c = 'z';
    let heart_eyed_cat = '😻';

    //compound types
    //tuple, array

    //tuple
    let tup: (i32, f64, u8) = (500, 6.3, 1);
    println!("{:?}", tup);

    let (x, y, z) = tup;
    println!("{x}, {y}, {z} ---- {:?}", tup);
    let x = tup.0;

    //array type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5];

    println!("a: {}", a[0]);

    println!("Please enter an array index (0 - 4)");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered is not number");

    let element = a[index];
    println!("The value of the element at index {index}  is {element}");
}
