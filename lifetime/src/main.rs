fn longest(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        b;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("smaller");
    {
        let str2 = String::from("longer");
        longest_str = longest(&str1, &str2);
    }
    println!("{}", longest_str)
}
