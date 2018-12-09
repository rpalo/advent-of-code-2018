/// Day 9: Marble Mania
/// 
/// Figure out the scores of elves playing marbles

/// Calculate what the highest player's score is
pub fn winning_score(number_of_players: usize, max_points: usize) -> usize {
    let mut marbles: Vec<usize> = Vec::with_capacity(max_points - max_points/23);
    let mut players: Vec<usize> = Vec::new();
    for _i in 0..number_of_players {
        players.push(0);
    }

    marbles.push(0);
    marbles.push(1);
    let mut current = 1;
    for i in 2..=max_points {
        // println!("marbles: {:?}, current: {}", marbles, current);
        if i % 23 == 0 {
            players[i % number_of_players] += i;
            current = neg_mod_subtract(current, 7, marbles.len());
            players[i % number_of_players] += marbles.remove(current);
        } else {
            current = (current + 2) % (marbles.len());
            if current == 0 {
                marbles.push(i);
                current = marbles.len() - 1;
            } else {
                marbles.insert(current, i);
            }
        }
    }

    *players.iter().max().unwrap()
}

fn neg_mod_subtract(num: usize, subtractor: usize, modulus: usize) -> usize {
    match num.checked_sub(subtractor) {
        Some(result) => result,
        None => modulus - (subtractor - num),
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