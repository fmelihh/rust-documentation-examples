fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisiable by 3");
    } else {
        println!("not found");
    }

    let condition = true;
    let number = if condition { 5 } else { 3 };
    println!("{number}");

    let mut counter = 0;
    let result = loop {
        println!("counter: {counter}");
        counter += 1;
        if counter == 15 {
            break counter * 10;
        }
    };

    println!("counter {counter}, result (counter * 10) {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let a = [10, 1, 2, 3, 4];
    let mut index = 0;

    while index < a.len() {
        println!("array value is {}", a[index]);
        index += 1
    }

    for element in a {
        println!("for loop array value is {element}");
    }

    for number in (1..4).rev() {
        print!("{number} ")
    }
}
