fn main() {
    println!("Hello, world!");
    another_function(2, 'h');

    let y = 6;
    let y = {
      let x = 3;
        x + 10 // expression
        // x + 10; // statement
    };

    println!("{y}");

    let five_value = plus_one(five());
    println!("five_value + plus_one {five_value}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, unit_label: char) {
    println!("Another hello world !!!");
    println!("The parameter passed x: {x}, unit_label: {unit_label}");
}

