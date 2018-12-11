// #[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod day10;

fn main() {

    let mut input = File::open("data/day10.txt").expect("File not found.");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Couldn't read file.");

    let mut sky = day10::Sky::from_text(&contents);
    for i in 0..200000 {
        sky.update();
        sky.display();
    }
    
}
