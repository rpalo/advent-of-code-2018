/// Day 12: Subterranean Sustainability
/// 
/// Figure out which plants are growing based on the plants around them

use std::collections::HashMap;

pub struct Plants {
    pub plants: Vec<char>,
    pub rules: HashMap<Vec<char>, char>,
    pub zero: isize
}

impl Plants {
    pub fn new(initial: &str, rules: &str) -> Self {
        let mut plants = Self { 
            plants: initial.chars().collect(),
            rules: HashMap::new(),
            zero: 0,
        };
        for line in rules.lines() {
            let mut parts = line.split(" => ");
            let input = parts.next().unwrap();
            let output = parts.next().unwrap();
            plants.rules.insert(input.chars().collect(), output.chars().next().unwrap());
        }
        plants
    }

    pub fn iterate(&mut self, times: usize) {
        for t in 0..times {
            if self.plants.iter().take(5).any(|c| *c == '#') {
                for _i in 0..5 { self.plants.insert(0, '.'); }
                self.zero += 5;
            }
            if self.plants.iter().rev().take(5).any(|c| *c == '#') {
                for _i in 0..5 { self.plants.push('.'); }
            }
            let mut new_plants = self.plants.clone();
            for i in 2..(self.plants.len() - 2) {
                let neighborhood = &self.plants[i - 2..=i+2];
                new_plants[i] = *self.rules.get(neighborhood).unwrap_or(&'.');
            }
            self.plants = new_plants;
        }
    }

    pub fn live_count(&self) -> isize {
        self.plants.iter().enumerate()
            .filter(|(_index, plant)| **plant == '#')
            .map(|(index, _plant)|{
                (index as isize) - self.zero
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut plants = Plants::new("#..#.#..##......###...###", "...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #");
        plants.iterate(20);
        assert_eq!(325, plants.live_count());
    }
}