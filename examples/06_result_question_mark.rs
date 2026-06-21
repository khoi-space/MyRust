// Day 6: Result with '?'

fn calculate_per_person_cost(total_cost: f32, people_count: f32) -> Result<f32, String> {
    if people_count == 0.0 {
        Err(String::from("Cannot divide by 0! Group must have at least 1 person."))
    } else {
        Ok(total_cost / people_count)
    }
}

fn generate_invoice(total_cost: f32, people_count: f32) -> Result<String, String> {
    let cost = calculate_per_person_cost(total_cost, people_count)?;
    Ok(format!("Invoice created. Cost per person: ${cost}"))
}

fn main() {
    let booking_result = generate_invoice(5000.0, 10.0);
    match booking_result {
        Ok(invoice) => println!("{invoice}"),
        Err(error) => println!("Booking Error: {error}"),
    }
}