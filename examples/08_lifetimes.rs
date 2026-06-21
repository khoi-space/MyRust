// Day 10: Lifetimes management

fn print_msg(s: &str) { println!("{}", s); }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn extract_info<'a, 'b>(data: &'a str, prefix: &'b str) -> &'a str {
    &data[prefix.len()..]
}

fn main() {
    let x = "1234";
    let y = "123";
    let longest_str = longest(x, y);

    let config_str: &'static str = "BAUD_RATE:9600";

    // Level 1: 
    print_msg("Lifetime annotations");

    println!("Longest string is {}", longest_str);
}