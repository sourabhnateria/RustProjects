struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}
fn main() {
    let rect: Rect = Rect {
        width: 30,
        height: 40,
    };
    println!("the area of rectangle is {}", rect.area());
    println!("the perimeter of rectangle is {}", rect.perimeter());
}
