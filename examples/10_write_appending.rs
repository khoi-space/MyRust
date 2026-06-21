// Day 9: Write file appending

use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("examples/logging.txt")?;

    writeln!(file, "Second line.")?;

    Ok(())
}