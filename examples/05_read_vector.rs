// Day 5: Read vector

fn main() {
    let v = vec![10, 20, 30];

    // Cách 1: Dùng dấu ngoặc vuông (Nguy hiểm!)
    let element = v[0]; // Nếu vị trí vượt ra ngoài size -> crash
    println!("Element: {element}");

    // Cách 2: Dùng .get() (An toàn - Trả về một Option)
    match v.get(100) {
        Some(value) => println!("Element: {value}"),
        None => println!("Index is out of range!"),
    }
}