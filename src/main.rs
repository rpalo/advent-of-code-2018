use std::fs::File;
use std::io::prelude::*;

mod day1;

fn main() {

    let mut input = File::open("data/day1.txt").expect("File not found.");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Couldn't read file.");
    println!("{}", day1::final_frequency(&contents));
}
