//Day 2: Data types - Tuples
fn main() {
    let tup: (i32, f64, char) = (500, 6.4, '🥸');
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    println!("x={x} | y={y} | z={z} | five_hundred={five_hundred}");
}