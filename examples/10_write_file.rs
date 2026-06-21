// Day 9: Write file

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Hàm trả về một Result, ta dùng dấu '?' để truyền lỗi nhanh nếu không tạo được file.
    let mut file = File::create("examples/logging.txt")?;

    writeln!(file, "First line.")?;
    
    Ok(())
}