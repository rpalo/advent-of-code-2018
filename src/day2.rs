use std::collections::HashMap;

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
pub fn checksum(ids: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    ids.lines()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_two() {
        let c = Counter::new("aabbccddd");
        assert_eq!(3, c.count_value(2));
    }

    #[test]
    fn basic_ids() {
        let ids = "abcdef\n \
                    bababc\n \
                    abbcde\n \
                    abcccd\n \
                    aabcdd\n \
                    abcdee\n \
                    ababab\n";
        assert_eq!(12, checksum(ids));
    }
}