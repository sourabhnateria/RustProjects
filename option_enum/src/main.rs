// pub enum Option<T> {
//     None,
//     Some(T),
// }

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}
fn main() {
    let my_string = String::from("Raman");
    match find_first_a(my_string) {
        Some(index) => println!("the letter 'a' is found at index : {}", index),
        None => println!("the letter 'a' is not found in the string."),
    }
}
