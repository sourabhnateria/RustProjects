struct User {
    name: String,
    age: u32,
    active: bool,
}
fn main() {
    let user = User {
        name: String::from("Rohit"),
        age: 30,
        active: true,
    };
    println!(
        "{} is {} year old and he is active: {}",
        user.name, user.age, user.active
    )
}
