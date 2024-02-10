fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    let x = 5;
    s = ovnership_test(s);
    println!("{s}");
    let y = x;

    let mut s1 = String::from("hello");
    let mut s2 = s1.clone();

    s2.push_str("var");

    println!("{}, world!, {}", s1, s2);
}

fn ovnership_test(var: String) -> String{
    println!("a");
    var
}
