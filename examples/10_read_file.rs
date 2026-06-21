// Day 9: Read file

use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("examples/data.txt")?;

    println!("--- Data Read ---");
    println!("{}", content);

    Ok(())
}