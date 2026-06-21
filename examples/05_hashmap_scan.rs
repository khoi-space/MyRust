// Day 5: Scanning in HashMap

use std::collections::HashMap;

fn main() {
    let mut menu = HashMap::new();
    menu.insert("Tea", 15000);
    menu.insert("Coffee", 20000);
    menu.insert("Matcha Latte", 50000);
    menu.insert("Orange Juice", 30000);

    for (drink, cost) in &menu {
        println!("{drink} costs {cost} VND");
    }
}