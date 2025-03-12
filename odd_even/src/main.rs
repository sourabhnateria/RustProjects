fn is_even(n: i32) -> bool {
    n % 2 == 0
}
fn main() {
    let x = 99;
    let is_even = is_even(x);
    if is_even {
        print!("{} is even", x);
    } else {
        print!("{} is odd", x);
    }
}
