/// Day 5: Alchemical Reduction

use std::collections::HashSet;

// Part 1: How many units remain after fully reacting a polymer

pub fn reduce(polymer: &str, drop_mer: Option<char>) -> String {
    let mut result: Vec<char> = polymer.chars().collect();
    let mut current = 0;
    while current < result.len() - 1 {
        let this_char = result[current];
        let next_char = result[current + 1];
        if matches_drop_mer(this_char, drop_mer) {
            result.remove(current);
            if current != 0 { current -= 1; }
            continue;
        }
        if is_polymer_pair(this_char, next_char) {
            result.remove(current);
            result.remove(current);
            if current != 0 { current -= 1; }
            continue;
        }
        current += 1;
    }
    result.into_iter().collect()
}

fn is_polymer_pair(first: char, second: char) -> bool {
    (first.is_lowercase() && second.is_uppercase() && 
        second.to_lowercase().next().unwrap() == first) ||
    (first.is_uppercase() && second.is_lowercase() &&
        second.to_uppercase().next().unwrap() == first)
}

fn matches_drop_mer(c: char, drop_mer: Option<char>) -> bool {
    drop_mer.is_some()
    && c.to_lowercase().next().unwrap() == drop_mer.unwrap().to_lowercase().next().unwrap()
}

// Part 2: Figure out which polymer, when removed, allows the most
// compacting, remove it, and return the length of the shortest polymer
// after compaction.

/// Optimizes a polymer by figuring out which *one* pair is causing
/// the most problems and removing it.  The compacted string is returned
pub fn optimize(polymer: &str) -> String {
    let possible_mers: HashSet<char> = polymer.to_lowercase().chars().collect();
    let mut result_candidates: Vec<String> = Vec::new();

    for mer in possible_mers.into_iter() {
        result_candidates.push(reduce(polymer, Some(mer)));
    }

    result_candidates.into_iter()
        .min_by_key(|candidate| candidate.len())
        .expect("No result candidates found.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let polymer = "dabAcCaCBAcCcaDA";
        assert_eq!(10, reduce(polymer, None).len());
    }

    #[test]
    fn test_pair_at_beginning() {
        let polymer = "dDaaabb";
        assert_eq!(5, reduce(polymer, None).len())
    }

    #[test]
    fn test_part_two() {
        let polymer = "dabAcCaCBAcCcaDA";
        assert_eq!(4, optimize(polymer).len());
    }
}

