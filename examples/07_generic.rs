// Day 7: Generic

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Point<T> {
    fn display(&self) {
        println!("Point [{} {}]", self.x, self.y);
    }
}

fn main() {
    let point1 = Point { x: 5, y: 10 };
    let point2 = Point { x: 1.5, y: 10.5 };

    point1.display();
    point2.display();
}