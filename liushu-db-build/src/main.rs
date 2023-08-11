use std::env;

use liushu_core::dict::build;

fn main() {
    let inputs: Vec<String> = env::args().skip(1).collect();
    build(inputs, "./", "sunman").unwrap();
}
