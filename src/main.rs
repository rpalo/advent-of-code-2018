// #[macro_use] extern crate lazy_static;
//extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod day6;

fn main() {

    let mut input = File::open("data/day6.txt").expect("File not found.");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Couldn't read file.");
    println!("{}", day6::squares_closer_than(&contents, 10000));
}
