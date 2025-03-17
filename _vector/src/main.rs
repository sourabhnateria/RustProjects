fn main() {
    let mut vec = Vec::new();
    vec.push(1); // Use integers instead of chars
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);

    let ans = even_filter(&vec);
    println!("{:?}", ans);
}

fn even_filter(vec: &[i32]) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for &val in vec {
        // Use &val to avoid explicit dereferencing
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    new_vec
}
