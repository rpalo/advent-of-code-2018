/// Day 6: Chronal Coordinates
/// 
/// Calculate Manhattan Distances on the X-Y plane

use std::collections::HashMap;
use std::collections::HashSet;

// Part 1: Find the size of the largest non-infinite area'

/// A grid of X-Y coordinates and unclaimed points
/// 
/// coords is a vector of Coordinates.  Their index is their "ID number"
/// points is a vector of unclaimed points.  Their value is the ID of the 
///     closest Coordinate
struct Grid {
    coords: Vec<Coordinate>,
    points: Vec<Option<usize>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self { coords: vec![], points: vec![], width: 0, height: 0 }
    }

    pub fn from_text(text: &str) -> Self {
        let mut grid = Grid::new();
        for line in text.lines() {
            let mut coord = Coordinate::from_str(line);
            coord.id = grid.coords.len();
            grid.coords.push(coord);
        }
        let (height, width) = grid.bounds();
        grid.height = height;
        grid.width = width;
        grid.points.resize(width*height, None);
        grid.calculate_closest_coords();
        grid
    }

    fn bounds(&self) -> (usize, usize) {
        let max_row = self.coords.iter().map(|coord| coord.y).max().unwrap();
        let max_col = self.coords.iter().map(|coord| coord.x).max().unwrap();
        (max_row, max_col)
    }

    fn calculate_closest_coords(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut min_dist = self.width + self.height;
                for coord in self.coords.iter() {
                    let dist = coord.manhattan_distance_to(x + 1, y + 1);
                    if dist < min_dist {
                        min_dist = dist;
                        self.points[x + y*self.width] = Some(coord.id);
                    } else if dist == min_dist {
                        // It's a tie.  No one gets it
                        self.points[x + y*self.width] = None;
                    }
                }
            }
        }
    }

    fn is_internal(&self, id: usize) -> bool {
        let mut external: HashSet<usize> = HashSet::new();
        // Left and right side
        for y in 0..self.height {
            let left = self.points[0 + y*self.width];
            let right = self.points[y*self.width + self.width - 1];
            if left.is_some() { external.insert(left.unwrap()); }
            if right.is_some() { external.insert(right.unwrap()); }
        }

        // Top and bottom
        for x in 0..self.width {
            let top = self.points[x];
            let bottom = self.points[x + (self.height - 1)*self.width];
            if top.is_some() { external.insert(top.unwrap()); }
            if bottom.is_some() { external.insert(bottom.unwrap()); }
        }

        !external.contains(&id)
    }

    pub fn most_claimed_area(&self) -> usize {
        let mut counter: HashMap<usize, usize> = HashMap::new();
        for point in self.points.iter() {
            if point.is_some() {
                *counter.entry(point.unwrap()).or_insert(0) += 1;
            }
        }
        *counter.iter()
            .filter(|(id, _count)| self.is_internal(**id))
            .map(|(_id, count)| count)
            .max().unwrap()
    }

    pub fn squares_closer_than(&self, dist: usize) -> usize {
        let mut distances: Vec<usize> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let total = self.coords.iter()
                    .fold(0, |acc, coord| acc + coord.manhattan_distance_to(x, y));
                if total < dist {
                    distances.push(total);
                }
            }
        }
        distances.iter().count()
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
        Self { id: 0, x, y }
    }

    pub fn from_str(text: &str) -> Self {
        let mut parts = text.split(',');
        let x = parts.next().unwrap().trim().parse().unwrap();
        let y = parts.next().unwrap().trim().parse().unwrap();
        Self { id: 0, x, y }
    }

    pub fn manhattan_distance_to(&self, x: usize, y: usize) -> usize {
        let x1 = self.x as i32;
        let x2 = x as i32;
        let y1 = self.y as i32;
        let y2 = y as i32;
        ((x2 - x1).abs() + (y2 - y1).abs()) as usize
    }
}

pub fn largest_finite_area(text: &str) -> usize {
    let grid = Grid::from_text(text);
    grid.most_claimed_area()
}

pub fn squares_closer_than(text: &str, dist: usize) -> usize {
    let grid = Grid::from_text(text);
    grid.squares_closer_than(dist)
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

    #[test]
    fn test_part_two() {
        let coords = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

        assert_eq!(16, squares_closer_than(coords, 32));
    }
}