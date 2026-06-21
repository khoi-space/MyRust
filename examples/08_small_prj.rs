// Day 10: Small project

struct PacketParser<'a> {
    raw_payload: &'a str,
}

impl <'a> PacketParser<'a> {
    fn extract_value<'b>(&self, filter: &'b str) -> &'a str {
        // Hàm trích xuất giá trị dựa trên một từ khóa tạm thời (filter_key)
        if let Some(pos) = self.raw_payload.find(filter) {
            let start_idx = pos + filter.len();
            &self.raw_payload[start_idx..]
        } else {
            "NOT_FOUND"
        }
    }
}

fn main() {
    let uart_buffer = String::from("DEVICE:ROBOT_01:BAUD:9600");
    let parser = PacketParser { raw_payload: &uart_buffer };

    let baud_rate;
    {
        let filter_key = String::from("BAUD:");

        baud_rate = parser.extract_value(&filter_key);
    }

    println!("Baudrate: {baud_rate}");
}