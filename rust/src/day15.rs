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
    health: usize,
}

type Grid = Vec<Vec<Position>>;

pub fn find_nearest_reachable(map: Grid, start: Position) -> Option<Position> {
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

pub fn best_next_step(map: Grid, start: Position, target: Position) -> Position {
    let mut results: Vec<(usize, usize)> = Vec::new();
    let neighbors: Vec<Position> = vec![
        map[start.y-1][start.x],
        map[start.y][start.x-1],
        map[start.y][start.x+1],
        map[start.y+1][start.x],
    ];

    for (i, neighbor) in neighbors.iter().enumerate() {
        if neighbor.value == '.' {
            results.push((distance(*neighbor, target), i));
        }
    }
    let index = results.iter().min().unwrap().1;
    neighbors[index]
}

pub fn select_attack_target(map: Grid, attacker: Position) -> Position {
    let neighbors: Vec<Position> = vec![
        map[attacker.y-1][attacker.x],
        map[attacker.y][attacker.x-1],
        map[attacker.y][attacker.x+1],
        map[attacker.y+1][attacker.x],
    ];
    neighbors.into_iter().enumerate().min_by_key(|(i, pos)| (pos.health, *i)).unwrap().1
}

pub fn distance(start: Position, end: Position) -> usize {
    ((end.x as isize - start.x as isize).abs() + (end.y as isize - start.y as isize).abs()) as usize
}

pub fn parse_map(text: &str) -> Vec<Vec<Position>> {
    let mut result: Vec<Vec<Position>> = Vec::new();
    for (y, row) in text.lines().enumerate() {
        result.push(Vec::new());
        for (x, c) in row.chars().enumerate() {
            result[y].push(Position{x: x, y: y, value: c, health: 200});
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
                    Position{x: 0, y: 0, value: '#', health: 200},
                    Position{x: 1, y: 0, value: '#', health: 200},
                    Position{x: 2, y: 0, value: '#', health: 200},
                    Position{x: 3, y: 0, value: '#', health: 200},
                ],
                vec![
                    Position{x: 0, y: 1, value: '#', health: 200},
                    Position{x: 1, y: 1, value: '.', health: 200},
                    Position{x: 2, y: 1, value: 'E', health: 200},
                    Position{x: 3, y: 1, value: '#', health: 200},
                ],
                vec![
                    Position{x: 0, y: 2, value: '#', health: 200},
                    Position{x: 1, y: 2, value: '#', health: 200},
                    Position{x: 2, y: 2, value: '#', health: 200},
                    Position{x: 3, y: 2, value: '#', health: 200},
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
        let start = Position{x: 1, y: 1, value: 'E', health: 200};
        assert_eq!(
            Some(Position{x: 3, y: 1, value: '.', health: 200}),
            find_nearest_reachable(map, start)
        );
    }

    #[test]
    fn test_choose_best_next_step() {
        let text = "
#######
#.E...#
#.....#
#...G.#
#######
".trim();
        let map = parse_map(&text);
        let start = Position{x: 2, y: 1, value: 'E', health: 200};
        let target = Position{x: 4, y: 2, value: '.', health: 200};
        assert_eq!(
            Position{x: 3, y: 1, value: '.', health: 200},
            best_next_step(map, start, target)
        );
    }

    #[test]
    fn test_attack_weakest_reading() {
        let text = "
#######
#G....#
#..G..#
#..EG.#
#..G..#
#######
".trim();
        let mut map = parse_map(&text);
        map[2][3] = Position{x: 3, y: 2, value: 'G', health: 4};
        let expected = Position{x: 4, y: 3, value: 'G', health: 2};
        map[3][4] = expected;
        map[4][3] = Position{x: 3, y: 4, value: 'G', health: 2};
        let target: Position = select_attack_target(map, Position{x: 3, y: 3, value: 'E', health: 200});
        assert_eq!(target, expected);
    }
}