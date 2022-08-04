#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.width > other.width
    }
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("area is {}", rect1.area());
    if rect1.width() {
        println!(
            "The rectangle has nonzero width. The width is {}",
            rect1.width
        );
    }
    println!("Can rect1 holds rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 holds rect3? {}", rect1.can_hold(&rect3));
}
