struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

struct Color(i32, i32, i32); // tuple struct

struct AlwaysEqual; // unit type struct


fn main() {
    let user1 = User {
        active: true,
        email: String::from("abc"),
        sing_in_count: 0,
        username: String::from("test"),
    };

    //override
    let user2 = User {
        email: String::from("test"),
        ..user1
    };

    // println!("{}, {}", user1.username, user2.active);
    // user1 -> username we can no longer use because is a reference type.

    let black = Color(0, 0, 0); // tuple struct usage

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        sing_in_count: 0,
        username,
    }
}
