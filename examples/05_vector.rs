// Day 5: Vector

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1); // Thêm phần tử vào cuối
    v1.push(2);

    let mut v2 = vec![10, 20, 30];
    v2.pop();
    v2.push(40);

    println!("v1={:?}", v1);
    println!("v2={:?}", v2);
}