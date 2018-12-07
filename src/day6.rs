/// Day 6: Chronal Coordinates
/// 
/// Calculate Manhattan Distances on the X-Y plane

use std::collections::HashMap;

// Part 1: Find the size of the largest non-infinite area'

/// A grid of X-Y coordinates and unclaimed points
/// 
/// coords is a vector of Coordinates.  Their index is their "ID number"
/// points is a vector of unclaimed points.  Their value is the ID of the 
///     closest Coordinate
struct Grid {
    coords: Vec<Coordinate>,
    points: Vec<Option<usize>>,
}

impl Grid {
    pub fn new() -> Self {
        Self { coords: vec![], points: vec![] }
    }

    pub fn from_text(text: &str) -> Self {
        let mut grid = Grid::new();
        for line in text.lines() {
            let coord = Coordinate::from_str(line);
            grid.coords.push(coord);
        }
        let (north, south, east, west) = grid.bounds();
        for y in north..=south {
            for x in east..=west {
                grid.points.push(None);
            }
        }
        grid.calculate_closest_coords();
        grid
    }

    fn bounds(&self) -> (usize, usize, usize, usize) {
        let north = self.coords.iter().map(|coord| coord.y).min().unwrap();
        let south = self.coords.iter().map(|coord| coord.y).max().unwrap();
        let east = self.coords.iter().map(|coord| coord.x).min().unwrap();
        let west = self.coords.iter().map(|coord| coord.x).max().unwrap();
        (north, south, east, west)
    }

    fn calculate_closest_coords(&mut self) {
        
    }
}

/// An X-Y coordinate on a Grid
struct Coordinate {
    id: usize,
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn from_str(text: &str) -> Self {
        let mut parts = text.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Self { x, y }
    }

    pub fn manhattan_distance_to(&self, x: usize, y: usize) -> usize {
        (((self.x - x) as i32).abs() + ((self.y - y) as i32).abs()) as usize
    }
}

pub fn largest_finite_area(text: &str) -> usize {
    let grid = Grid::from_text(text);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let coords = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

        assert_eq!(17, largest_finite_area(coords));
    }
}