/// Day 15: Beverage Bandits
///
/// Simulate a battle between elves and goblins

use std::collections::VecDeque;
use std::collections::HashMap;

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
    health: isize,
}

type Grid = Vec<Vec<Position>>;

enum Round {
    Complete(Grid),
    Partial(Grid),
}

pub fn ground(x: usize, y: usize) -> Position {
    Position {x: x, y: y, value: '.', health: 0}
}

fn best_next_step(map: &Grid, start: Position) -> Option<Position> {
    let mut q: VecDeque<Vec<Position>> = VecDeque::new();
    q.push_back(vec![start]);
    let mut seen: HashMap<Position, usize> = HashMap::new();

    let enemy: char = match start.value {
        'E' => 'G',
        'G' => 'E',
        _ => '?',
    };
    while !q.is_empty() {
        let current_chain = q.pop_front().unwrap();
        let current = current_chain[current_chain.len() - 1];
        
        let neighbors: Vec<Position> = vec![
            map[current.y-1][current.x],
            map[current.y][current.x-1],
            map[current.y][current.x+1],
            map[current.y+1][current.x],
        ];
        for neighbor in neighbors {
            if neighbor.value == enemy {
                return Some(current_chain[1]);
            }
            if neighbor.value == '.' && (seen.get(&neighbor).is_none() || *seen.get(&neighbor).unwrap() > current_chain.len() + 1) {
                let mut new_spot = current_chain.clone();
                new_spot.push(neighbor);
                seen.insert(neighbor, current_chain.len() + 1);
                q.push_back(new_spot);
            }
        }
        //println!("Searching move. Q Length {}", q.len());
    }

    return None;
}


fn any_warriors(map: &Grid, team: char) -> bool {
    map.iter().flatten().any(|pos| pos.value == team)
}

fn can_attack(map: &Grid, attacker: Position) -> bool {
    let enemy = match attacker.value {
        'E' => 'G',
        'G' => 'E',
        _ => '?',
    };
    let neighbors: Vec<Position> = vec![
        map[attacker.y-1][attacker.x],
        map[attacker.y][attacker.x-1],
        map[attacker.y][attacker.x+1],
        map[attacker.y+1][attacker.x],
    ];
    neighbors.iter().any(|&neighbor| neighbor.value == enemy)
}

fn select_attack_target(map: &Grid, attacker: Position) -> Position {
    let neighbors: Vec<Position> = vec![
        map[attacker.y-1][attacker.x],
        map[attacker.y][attacker.x-1],
        map[attacker.y][attacker.x+1],
        map[attacker.y+1][attacker.x],
    ];
    let enemy = match attacker.value {
        'E' => 'G',
        'G' => 'E',
        _ => '?',
    };
    neighbors.into_iter().enumerate()
        .filter(|(_i, pos)| pos.value == enemy)
        .min_by_key(|(i, pos)| (pos.health, *i))
        .unwrap().1
}

fn simulate_round(map: Grid) -> Round {
    let mut next_round = map.clone();
    let characters = map.iter().flatten().filter(|pos| pos.value == 'E' || pos.value == 'G');
    for ref_character in characters {
        let mut character = *ref_character;

        if next_round[character.y][character.x].value == '.' {
            // Character has already died.  Skip turn, don't let him attack.
            continue;
        }

        let enemy_team = match character.value {
            'E' => 'G',
            'G' => 'E',
            _ => '?',
        };
        
        if ! any_warriors(&next_round, enemy_team) {
            return Round::Partial(next_round);
        }

        if !can_attack(&next_round, character) {
            let next_step = best_next_step(&next_round, character);

            if next_step.is_none() {
                continue;
            }

            let next_step = next_step.unwrap();
            next_round[character.y][character.x] = ground(character.x, character.y);
            character = Position{x: next_step.x, y: next_step.y, value: character.value, health: character.health};
            next_round[character.y][character.x] = character;
        }

        if !can_attack(&next_round, character) {
            continue;
        }

        let target = select_attack_target(&next_round, character);
        let new_target = Position{x: target.x, y: target.y, value: target.value, health: target.health - 3};
        if new_target.health <= 0 {
            next_round[target.y][target.x] = ground(target.x, target.y);
        } else {
            next_round[target.y][target.x] = new_target;
        }
    }
    
    return Round::Complete(next_round);
    
}

pub fn simulate_battle(text: &str) -> isize {
    let mut map = parse_map(text);
    let mut still_battling = true;
    let mut rounds = 0;
    let mut last_elf_gob = (0, 0);
    while still_battling {
        match simulate_round(map) {
            Round::Complete(new_map) => {
                map = new_map;
                rounds += 1;
            },
            Round::Partial(final_map) => {
                map = final_map;
                still_battling = false;
            }
        }
    }

    let total_hitpoints: isize = map.iter().flatten().filter(|pos| pos.value == 'E' || pos.value == 'G')
        .map(|pos| pos.health).sum();

    rounds * total_hitpoints
}

fn distance(start: Position, end: Position) -> usize {
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

pub fn print_map(map: &Grid) {
    for row in map {
        for pos in row {
            print!("{}", pos.value);
        }
        println!("");
    }
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
        assert_eq!(
            Position{x: 3, y: 1, value: '.', health: 200},
            best_next_step(&map, start).unwrap()
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
        let target: Position = select_attack_target(&map, Position{x: 3, y: 3, value: 'E', health: 200});
        assert_eq!(target, expected);
    }

    #[test]
    fn test_full_battle() {
        let text = "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
".trim();
        assert_eq!(27730, simulate_battle(text));
    }

    #[test]
    fn test_full_battle2() {
        let text = "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
".trim();
        assert_eq!(36334, simulate_battle(text));
    }

        #[test]
    fn test_full_battle3() {
        let text = "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
".trim();
        assert_eq!(39514, simulate_battle(text));
    }

        #[test]
    fn test_full_battle4() {
        let text = "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
".trim();
        assert_eq!(27755, simulate_battle(text));
    }

        #[test]
    fn test_full_battle5() {
        let text = "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
".trim();
        assert_eq!(28944, simulate_battle(text));
    }

        #[test]
    fn test_full_battle6() {
        let text = "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
".trim();
        assert_eq!(18740, simulate_battle(text));
    }

}