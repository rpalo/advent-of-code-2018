/// Day 5: Alchemical Reduction

// Part 1: How many units remain after fully reacting a polymer

pub fn reduce(polymer: &str) -> String {
    let mut result: Vec<char> = polymer.chars().collect();
    let mut current = 0;
    while current < result.len() - 1 {
        let this_char = result[current];
        let next_char = result[current + 1];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let polymer = "dabAcCaCBAcCcaDA";
        assert_eq!(10, reduce(polymer).len());
    }

    #[test]
    fn test_pair_at_beginning() {
        let polymer = "dDaaabb";
        assert_eq!(5, reduce(polymer).len())
    }
}

