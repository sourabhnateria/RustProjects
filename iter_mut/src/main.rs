fn main() {
    let mut num = vec![1, 2, 3];
    let iter = num.iter_mut();
    for value in iter {
        *value = *value + 1;
    }
    println!("{:?}", num);
}
