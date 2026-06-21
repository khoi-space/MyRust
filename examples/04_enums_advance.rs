// Day 4: Enum (Advanced)

#[derive(Debug)]
enum Status {
    Processing,
    Delivering(String),
    Delivered,
    Cancelled{ reason: String, code: u32 },
}

fn get_status_message(packet: Status) {
    match packet {
        Status::Processing => {
            println!("Shop is packet your packet. Please waiting...");
        }

        Status::Delivering(shipper) => {
            println!("Your packet is being shipped by: {shipper}");
        }

        Status::Delivered => {
            println!("Your packet was delivered sucessfully.");
        }

        Status::Cancelled { reason, code } => {
            println!("Your packet was cancelled. Reason: {reason}. Code: {code}");
        }
    }
}

fn main() {
    let packet1 = Status::Delivering(String::from("J&T Express"));
    let packet2 = Status::Cancelled { reason: String::from("Out of stock"), code: 404 };

    // println!("{:?}", packet1);
    // println!("{:?}", packet2);

    get_status_message(packet1);
    get_status_message(packet2);
}