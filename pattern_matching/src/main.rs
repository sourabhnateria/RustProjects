enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side_len) => side_len * side_len,
        Shape::Rectangle(width, height) => width * height,
    }
}
fn main() {
    let circle = Shape::Circle(5.2);
    let square = Shape::Square(6.3);
    let rectangle = Shape::Rectangle(1.5, 3.0);
    println!("area of circle is {} ", calculate_area(circle));
    println!("area of square is {} ", calculate_area(square));
    println!("area of rectangle is {} ", calculate_area(rectangle));
}
