// constant definitions
const MY_FIRST_CONST_VAR: u8 = 15;

fn main() {
    // mutable definition
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("My constant variable is {MY_FIRST_CONST_VAR}");

    let y = 5;
    //shadowing
    let y = y + 15;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is {y}");

    // shadowing difference. this code gives the compile error.
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let spaces = "  ";
    let spaces = spaces.len();

    println!("spaces {spaces}");
}
