/// Day 18: Settlers of the North Pole
/// 
/// Figure out how a North Pole logging operation evolves.

use std::collections::HashMap;

enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Clone)]
struct Logging {
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

        for y, row in self.spaces.iter().enumerate() {
            for x, acre in row.iter().enumerate() {
                let count: HashMap<Acre, usize> = self.demographics(x, y);
                match acre {
                    Acre::Open => {
                        if count.get(Acre::Trees).unwrap_or(0) >= 3 {
                            result[y][x] = Acre::Trees;
                        }
                    },
                    Acre::Trees => {
                        if count.get(Acre::Lumberyard).unwrap_or(0) >= 3 {
                            result[y][x] = Acre::Lumberyard;
                        }
                    },
                    Acre::Lumberyard {
                        if ! (count.contains_key(Acre::Trees) && count.contains_key(Acre::Lumberyard)) {
                            result[y][x] = Acre::Open;
                        }
                    },
                }
            }
        }

        result
    }

    fn demographics(&self, x, y) -> HashMap<Acre, usize> {
        let mut count: HashMap<Acre, usize> = HashMap::new();
        self.neighbors(x, y).iter().for_each(|neighbor| {
            let current = count.entry(neighbor).or_insert(0);
            *current += 1;
        });
        
        count
    }

    fn neighbors(&self, x, y) -> Vec<Acre> {
        let mut result: Vec<Acre> = Vec::new();

        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                match self.spaces.get(y+y_offset) {
                    Some(row) => {
                        match row.get(x+x_offset) {
                            Some(a) => result.push(a),
                            None => (),
                        }
                    },
                    None => (),
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_thing() {

    }
}