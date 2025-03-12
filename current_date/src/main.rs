use chrono::{Local, Utc};

fn main() {
    let now = Utc::now();
    println!("current date and time in UTC : {}", now);

    //format date and time
    let formatted = now.format("%y-%m-%d %H:%M:%S");
    println!("formatted date and time : {}", formatted);

    //get local time
    let local = Local::now();
    println!("current date and time in local :{}", local);
}
