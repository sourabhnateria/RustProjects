pub trait Summary {
    fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: u8,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("{} age is {} ", self.name, self.age);
    }
}

pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarise());
}

fn main() {
    let user = User {
        name: String::from("sourabh"),
        age: 28,
    };
    notify(&user);
}
