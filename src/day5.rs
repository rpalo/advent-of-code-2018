/// Day 5: Alchemical Reduction

use std::collections::HashSet;

// Part 1: How many units remain after fully reacting a polymer

/// Reduce down a polymer by dropping any mer pairs
/// 
/// A mer pair is any lowercase letter and its corresponding uppercase
/// letter, adjacent to each other
/// Optionally, provide a "drop_mer" to drop unconditionally, case insenitive
/// pair or not
pub fn reduce(polymer: &str, drop_mer: Option<char>) -> String {
    let mut result: Vec<char> = Vec::new();
    for c in polymer.chars() {
        if matches_drop_mer(c, drop_mer) { continue; }
        if result.last().is_some() && is_polymer_pair(*result.last().unwrap(), c) {
            result.pop();
        } else {
            result.push(c);
        }
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

/// Optimizes a polymer by figuring out which *one* mer is inhibiting
/// reduction the most and removing it.  The reduced string is returned
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

