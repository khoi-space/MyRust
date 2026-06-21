// Day 4: Implementation in struct

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }

    fn set_length(&mut self, new_length: u32) {
        self.length = new_length;
    }

    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }
}

fn main() {
    let mut rec = Rectangle { width: 30, length: 40};
    println!("Area of rectangle is {}", rec.area());

    rec.set_length(100);
    rec.set_width(500);
    println!("Area of rectangle is {}", rec.area());

    let other_rec = Rectangle::new(10, 40);
    println!("Area of rectangle is {}", other_rec.area());
}