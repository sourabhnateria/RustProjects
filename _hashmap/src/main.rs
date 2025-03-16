use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("sourabh"), 28);
    users.insert(String::from("raman"), 45);
    let first_user_age = users.get("raman");

    match first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("user not found in the db "),
    }
}
