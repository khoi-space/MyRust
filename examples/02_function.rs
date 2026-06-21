// Day 2: Function
fn main() {
    let result = sum(5, 10);
    println!("Sum={result}");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b // <= Không có dấu chấm phẩy ở đây!
}