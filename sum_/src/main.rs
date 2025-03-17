use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..10000000 {
                ans = ans + (i * 10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);

    let mut ans: u64 = 0;
    for val in rx {
        ans = ans + val;
    }
    println!("ans is {}", ans);
}
