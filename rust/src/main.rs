// #[macro_use] extern crate lazy_static;
// extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod day15;

fn main() {
    let text = "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
".trim();

    println!("{}", day15::simulate_battle(text));
}
