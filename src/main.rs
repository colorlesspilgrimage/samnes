use std::env;
use std::fs;

fn main() {
    let bin = fs::read(env::args().next_back().unwrap()).unwrap();

    for b in &bin {
        print!("{}", b);
    }
}
