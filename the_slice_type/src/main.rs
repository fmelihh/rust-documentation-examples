fn main() {
    let entire_word = String::from("asdasdasdsdsa");
    let sub_word = first_word(&entire_word);
    // entire_word.clear(); -> error occurred
    // because sub_word slice part of entire word.
    // if sub_word has mut reference
    // it will remove easily.

    println!("{sub_word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}

fn first_word(s: &str) -> &str {
    // Here’s a small programming problem: write a function that takes a string
    // of words separated by spaces and returns the first word it finds in that string.
    // If the function doesn’t find a space in the string, the whole string must be one word,
    // so the entire string should be returned

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
