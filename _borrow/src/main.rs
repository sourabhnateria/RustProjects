fn main() {
    let my_string: String = String::from("hello rust");
    takes_ownership(&my_string);
    println!("{}", my_string);
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string)
}
