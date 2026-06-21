// Day 5: HashMap

use std::collections::HashMap;

fn main() {
    let mut point = HashMap::new();

    point.insert(String::from("Blue"), 10);
    point.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    match point.get(&team_name) {
        Some(point) => println!("Team {team_name} has: {point} points."),
        None => println!("Team {team_name} not found.")
    }
}