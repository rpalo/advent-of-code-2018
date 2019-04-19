// #[macro_use] extern crate lazy_static;
// extern crate regex;

use std::fs;
use std::io::prelude::*;

mod day15;

fn main() {
    let text = fs::read_to_string("data/day15.txt").unwrap();

    println!("{}", day15::simulate_battle(&text));
}
