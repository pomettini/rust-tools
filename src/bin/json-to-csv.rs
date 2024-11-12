use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read(&args[1]).unwrap();

    dbg!(file);
}
