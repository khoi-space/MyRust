// Day 3 : Mutable reference

fn main() {
    let mut s = String::from("hello world");

    upper_str(&mut s);
    println!("{s}");
}

fn upper_str(str: &mut String) {
    str.make_ascii_uppercase();
}