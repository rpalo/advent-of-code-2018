/// Day 15: Beverage Bandits
///
/// Simulate a battle between elves and goblins

use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub struct Position {
    x: usize,
    y: usize,
    value: char,
}

pub fn find_nearest_reachable(map: Vec<Vec<Position>>, start: Position) -> Option<Position> {
    let mut q: VecDeque<Position> = VecDeque::new();
    q.push_back(start);
    let mut seen: HashSet<Position> = HashSet::new();

    let enemy: char = match start.value {
        'E' => 'G',
        'G' => 'E',
        _ => '?',
    };
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        seen.insert(current);
        
        let neighbors: Vec<Position> = vec![
            map[current.y-1][current.x],
            map[current.y][current.x-1],
            map[current.y][current.x+1],
            map[current.y+1][current.x],
        ];
        for neighbor in neighbors {
            if neighbor.value == enemy {
                return Some(current);
            } else if neighbor.value == '.' {
                q.push_back(neighbor);
            }
        }
    }

    return None;
}

// pub fn distance(start: Position, end: Position) -> usize {
//     (end.x - start.x).abs() + (end.y - start.y).abs()
// }

pub fn parse_map(text: &str) -> Vec<Vec<Position>> {
    let mut result: Vec<Vec<Position>> = Vec::new();
    for (y, row) in text.lines().enumerate() {
        result.push(Vec::new());
        for (x, c) in row.chars().enumerate() {
            result[y].push(Position{x: x, y: y, value: c});
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_parse_map() {
        let text = "
####
#.E#
####
".trim();
        assert_eq!(
            vec![
                vec![
                    Position{x: 0, y: 0, value: '#'},
                    Position{x: 1, y: 0, value: '#'},
                    Position{x: 2, y: 0, value: '#'},
                    Position{x: 3, y: 0, value: '#'},
                ],
                vec![
                    Position{x: 0, y: 1, value: '#'},
                    Position{x: 1, y: 1, value: '.'},
                    Position{x: 2, y: 1, value: 'E'},
                    Position{x: 3, y: 1, value: '#'},
                ],
                vec![
                    Position{x: 0, y: 2, value: '#'},
                    Position{x: 1, y: 2, value: '#'},
                    Position{x: 2, y: 2, value: '#'},
                    Position{x: 3, y: 2, value: '#'},
                ],
            ], parse_map(text)
        );
    }

    #[test]
    fn test_find_nearest_enemy() {
        let text = "
#######
#E..G.#
#...#.#
#.G.#G#
#######
".trim();
        let map = parse_map(&text);
        let start = Position{x: 1, y: 1, value: 'E'};
        assert_eq!(
            Some(Position{x: 3, y: 1, value: '.'}),
            find_nearest_reachable(map, start)
        );
    }
}