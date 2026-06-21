// Day 4 : Option Enum

fn main() {
    let gifted_box: Option<&str> = Some("TV");
    let empty_box: Option<&str> = None;

    match gifted_box {
        Some(stuff) => println!("Congratulation! You received: {stuff}"),
        None => println!("Sorry, this is an empty box"),
    }

    // empty_box.unwrap(); // Không nên vì nếu hộp rỗng (None) sẽ gây crash.

    let other_stuff = empty_box.unwrap_or("discount 5%");
    println!("Your box is empty but you will have {other_stuff}.");

    if let Some(stuff) = gifted_box {
        println!("Your gift is: {stuff}");
    }
}