// Day 2: Print / Println

fn main() {
    let name = "Rust";
    let version = 2026;

    // Cách 1: Truyền biến vào sau chuỗi
    println!("Welcome to {} version {}", name, version);

    // Cách 2: Truyền trực tiếp
    println!("Welcome to {name} version {version}");

    // Cách 3: Định vị bằng số thứ tự (index)
    println!("Welcome to {0} version {1}. {0} is easy to learn.", name, version);
}