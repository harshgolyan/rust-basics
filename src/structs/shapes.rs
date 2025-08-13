struct Rect {
    height: i32,
    width: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

fn main() {
    let rect1: Rect = Rect {
        height: 10,
        width: 20,
    };
    println!("Area of rectangle: {}", rect1.area());
}