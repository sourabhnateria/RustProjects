fn main() {
    let mut s1: String = String::from("hello");
    update_str(&mut s1);
    println!("{}", s1);
}

fn update_str(str: &mut String) {
    str.push_str("world");
}
