// Day 6: Error handling - Panic!

fn main() {
    println!("This line is still running...");

    panic!("Overheated. Terminating...");

    println!("This line won't run...");
}