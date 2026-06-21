// Day 4 : Struct

struct User {
    username: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        username: String::from("Nguyen Van A"),
        sign_in_count: 1,
    };

    user1.sign_in_count = 2;
    println!("User {} signed in {} times.", user1.username, user1.sign_in_count);
}