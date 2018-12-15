/// Day 9: Marble Mania
/// 
/// Figure out the scores of elves playing marbles

use std::collections::VecDeque;

/// Calculate what the highest player's score is
pub fn winning_score(number_of_players: usize, max_points: usize) -> usize {
    let mut marbles: VecDeque<usize> = VecDeque::new();
    let mut players: Vec<usize> = Vec::new();
    for _i in 0..number_of_players {
        players.push(0);
    }

    marbles.push_back(0);
    marbles.push_back(1);

    for i in 2..=max_points {
        if i % 100000 == 0 { println!("{}", i) };
        
        if i % 23 == 0 {
            players[i % number_of_players] += i;
            rotate_right(&mut marbles, 7);
            players[i % number_of_players] += marbles.pop_back().unwrap();
            rotate_left(&mut marbles, 1);
        } else {
            rotate_left(&mut marbles, 1);
            marbles.push_back(i);
        }
    }

    *players.iter().max().unwrap()
}

fn rotate_right(list: &mut VecDeque<usize>, times: usize) {
    for _i in 0..times {
        let value = list.pop_back().expect("Empty VecDeque");
        list.push_front(value);
    }
}

fn rotate_left(list: &mut VecDeque<usize>, times: usize) {
    for _i in 0..times {
        let value = list.pop_front().expect("Empty VecDeque");
        list.push_back(value);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_with_9() {
        assert_eq!(32, winning_score(9, 25));
    }

    #[test]
    fn test_part_one_with_10() {
        assert_eq!(8317, winning_score(10, 1618));
    }

    #[test]
    fn test_part_one_with_13() {
        assert_eq!(146373, winning_score(13, 7999));
    }

    #[test]
    fn test_part_one_with_17() {
        assert_eq!(2764, winning_score(17, 1104));
    }

    #[test]
    fn test_part_one_with_21() {
        assert_eq!(54718, winning_score(21, 6111));
    }

    #[test]
    fn test_part_one_with_30() {
        assert_eq!(37305, winning_score(30, 5807));
    }
}