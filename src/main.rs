#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod day3;

fn main() {

    let mut input = File::open("data/day3.txt").expect("File not found.");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Couldn't read file.");
    println!("{}", day3::count_conflicting_squares(&contents));
}
