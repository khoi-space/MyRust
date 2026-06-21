// Day 3 : Immutable reference

fn main() {
    let s1 = String::from("hello");

    let len = count_len(&s1);
    let s2 = &s1;

    println!("String '{}' has length {}", s1, len);
    println!("String s2 = {s2}");
}

fn count_len(s: &String) -> usize {
    s.len()
}