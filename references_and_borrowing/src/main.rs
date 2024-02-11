fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("Changed string is {s1}");

    /*
    Mutable references have one big restriction:
    if you have a mutable reference to a value, you can
    have no other references to that value.
    */
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // it throws an error.

    // Data races cause undefined behavior and can be difficult to diagnose and
    // fix when you’re trying to track them down at runtime;
    // Rust prevents this problem by refusing to compile code with data races!

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        // r1 goes out of scope here, so we can make a new reference with no problem.
    }

    let r2 = &mut s;

    let mut s10 = String::from("hello");
    let r99 = &s;
    let r31 = &s;
    // let r15 = &mut s; it throwns a error.


    println!("{}, {}", r99, r31);

    let r15 = &mut s;

    println!("{}", r15);


    // Whew! We also cannot have a mutable reference while we have an immutable one to the same value.


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", word");
}

// fn dangle() -> &String {
//     let s = String::from("asdasd");
//     &s
// } error: could not compile `ownership` due to previous error
// this function's return type contains a borrowed value, but there is no value
// for it to be borrowed from

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
// This works without any problems. Ownership is moved out, and nothing is deallocated.

// fn change(some_string: &String) {
//     some_string.push_str(", word");
// }
// could not compile `ownership` due to previous error
// some_string is not mutable in the default

