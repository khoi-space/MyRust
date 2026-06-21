// Day 6: Result

// Định nghĩa của Rust trong thư viện chuẩn:
// enum Result<T, E> {
//     Ok(T),  // Thành công: Trả về dữ liệu kiểu T
//     Err(E), // Thất bại: Trả về thông tin lỗi kiểu E
// }

fn divide(dividend: f32, divisor: f32) -> Result<f32, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by 0!"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result = divide(10.0, 0.0);

    match result {
        Ok(quotient) => println!("Result: {quotient}"),
        Err(error) => println!("Failed! Reason: {error}"),
    }
}