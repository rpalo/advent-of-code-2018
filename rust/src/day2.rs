use std::collections::HashMap;

// Part 1

/// A histogram of the characters in a string.
struct Counter {
    letters: HashMap<char, usize>,
}

impl Counter {
    pub fn new(word: &str) -> Self {
        let mut letters = HashMap::new();
        for letter in word.chars() {
            let current_reference = letters.entry(letter).or_insert(0);
            *current_reference += 1;
        }
        Self { letters }
    }

    pub fn count_value(&self, number: usize) -> usize {
        self.letters.values().filter(|count| **count == number).count()
    }
}

/// Calculates a checksum for an id string.
/// 
/// The checksum is the number of id's with at least one set of exactly
/// two of a letter times the number of id's with at least one set of
/// exactly three of a letter.  If it has more than one
pub fn checksum(text: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    text.lines()
        .map(|id| Counter::new(id))
        .for_each(|counter| {
            if counter.count_value(2) != 0 {
                twos += 1;
            }
            if counter.count_value(3) != 0 {
                threes += 1;
            }
        });
    twos * threes
}

// Part 2

/// Finds the letters that are shared between the two prototype fabric
/// box ids.
/// 
/// These ids are the only two that differ from each other by exactly
/// one letter.
pub fn prototype_ids_common_letters(text: &str) -> String {
    let ids: Vec<&str> = text.lines().collect();
    for (i, s1) in ids.iter().enumerate() {
        for s2 in ids.iter().skip(i) {
            if hamming_distance(s1, s2) == 1 {
                return common_letters(s1, s2);
            }
        }
    }
    String::new()
}

/// Calculates the "Hamming Distance" between two strings
/// 
/// Hamming distance is the number of characters who are different
/// between the two strings when the corresponding indices are compared
/// in each string
fn hamming_distance(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

/// Returns the letters that are the same (and in the same place)
/// between the two strings
fn common_letters(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _c2)| c1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1 Tests

    #[test]
    fn test_count_two() {
        let c = Counter::new("aabbccddd");
        assert_eq!(3, c.count_value(2));
    }

    #[test]
    fn basic_ids() {
        let ids = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        assert_eq!(12, checksum(ids));
    }

    // Part 2 Tests

    #[test]
    fn test_part_two() {
        let ids = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!(String::from("fgij"), prototype_ids_common_letters(ids));
    }
}