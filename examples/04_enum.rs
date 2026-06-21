// Day 4: Enum

#[derive(Debug)]
enum Status {
    Processing,
    Delivering,
    Delivered,
    Cancelled,
}

fn main() {
    let mut packet_status = Status::Processing;

    println!("Your packet is {:?}", packet_status);

    packet_status = Status::Cancelled;

    println!("Your packet is {:?}", packet_status);
}