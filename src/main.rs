use std::env;
use std::str::FromStr;

fn main() {
    let counter_str = env::args().nth(1).unwrap();
    let counter = i32::from_str(&counter_str).unwrap();

    for i in counter..10 {
        println!("{i}");
    }
}
