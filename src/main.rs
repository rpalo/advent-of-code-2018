// #[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod day12;

fn main() {

    let mut input = File::open("data/day12.txt").expect("File not found.");

    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Couldn't read file.");

    let mut plants = day12::Plants::new("##..#..##....#..#..#..##.#.###.######..#..###.#.#..##.###.#.##..###..#.#..#.##.##..###.#.#...#.##..", &contents);
    plants.iterate(50000000000);
    println!("{}", plants.live_count());
    
}
