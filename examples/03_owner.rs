// Day 3: Ownership

fn main() {
    {                               // Biến s chưa xuất hiện ở đây
        let s = "hello";      // s bắt đầu có hiệu lực kể từ dòng này
        println!("{s}");
    }                               // Phạm vi kết thúc, s "bốc hơi" khỏi bộ nhớ!
    // println!("{s}");
}