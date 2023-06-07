use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
}
