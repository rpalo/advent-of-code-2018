/// Day 18: Settlers of the North Pole
/// 
/// Figure out how a North Pole logging operation evolves.

use std::collections::HashMap;

#[derive(Clone)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Clone)]
pub struct Logging {
    spaces: Vec<Vec<Acre>>
}

impl Logging {
    pub fn new(text: &str) -> Self {
        let spaces: Vec<Vec<Acre>> = text
            .lines().map(|line| {
                line.chars().map(|c| {
                    match c {
                        '.' => Acre::Open,
                        '|' => Acre::Trees,
                        '#' => Acre::Lumberyard,
                        _ => panic!("Unknown symbol"),
                    }
                }).collect()
            }).collect();

        Self { spaces }
    }

    pub fn tick(&self) -> Self {
        let mut result = self.clone();

        for (y, row) in self.spaces.iter().enumerate() {
            for (x, acre) in row.iter().enumerate() {
                let count: HashMap<Acre, usize> = self.demographics(x, y);
                match acre {
                    Acre::Open => {
                        if *count.get(&Acre::Trees).unwrap_or(&0) >= 3 {
                            result.spaces[y][x] = Acre::Trees;
                        }
                    },
                    Acre::Trees => {
                        if *count.get(&Acre::Lumberyard).unwrap_or(&0) >= 3 {
                            result.spaces[y][x] = Acre::Lumberyard;
                        }
                    },
                    Acre::Lumberyard => {
                        if ! (count.contains_key(&Acre::Trees) && count.contains_key(&Acre::Lumberyard)) {
                            result.spaces[y][x] = Acre::Open;
                        }
                    },
                }
            }
        }

        result
    }

    pub fn resource_value(&self) -> usize {
        let mut trees= 0;
        let mut lumberyards = 0;
        self.spaces.iter().flatten().for_each(|space| {
            match space {
                Acre::Trees => trees += 1,
                Acre::Lumberyard => lumberyards += 1,
                Acre::Open => (),
            }
        });

        trees * lumberyards
    }

    fn demographics(&self, x: usize, y: usize) -> HashMap<Acre, usize> {
        let mut count: HashMap<Acre, usize> = HashMap::new();
        self.neighbors(x, y).into_iter().for_each(|neighbor| {
            let current = count.entry(neighbor).or_insert(0);
            *current += 1;
        });
        
        count
    }

    fn neighbors(&self, center_x: usize, center_y: usize) -> Vec<Acre> {
        let mut result: Vec<Acre> = Vec::new();

        for x_location in center_x.saturating_sub(1)..=center_x.saturating_add(1) {
            for y_location in center_y.saturating_sub(1)..=center_y.saturating_add(1) {
                if x_location == center_x && y_location == center_y {
                    continue;
                }

                match self.spaces.get(y_location) {
                    Some(row) => {
                        match row.get(x_location) {
                            Some(a) => result.push(a.clone()),
                            None => (),
                        }
                    },
                    None => (),
                }
            }
        }
        result
    }

    pub fn print_self(&self) {
        for row in self.spaces.iter() {
            for space in row {
                match space {
                    Acre::Open => print!("."),
                    Acre::Trees => print!("|"),
                    Acre::Lumberyard => print!("#"),
                }
            }
            println!("");
        }
        println!("====");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_value1() {
        let text = "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
".trim();
        let mut logging = Logging::new(text);
        for _ in 0..10 {
            logging = logging.tick();
        }
        assert_eq!(1147, logging.resource_value());
    }
}