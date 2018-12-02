use std::fs::File;
use std::io::prelude::*;

mod day2;

fn main() {

    let mut input = File::open("data/day2.txt").expect("File not found.");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Couldn't read file.");
    println!("{}", day2::prototype_ids_common_letters(&contents));
}
