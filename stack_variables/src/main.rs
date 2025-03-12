fn main() {
    let x: i32 = 1;
    let y: i32 = 5;
    println!("the sum of x and y is {}", sum(x, y));
    println!("hello world this is example of stack variables");
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}
