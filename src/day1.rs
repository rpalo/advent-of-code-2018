/// Given a bunch of integers (changes in frequency), one per line,
/// calculate the final sum
pub fn final_frequency(text: &str) -> i32 {
    text.lines()
        .map(|value: &str| -> i32 {value.parse().expect("Not a number")})
        .sum()
}

/// Given a bunch of integers (changes in frequency), one per line,
/// calculate the running sum until you hit a cumulative sum that you've
/// seen before.  Return that first repeated cumulative sum.
pub fn first_duplicate_frequency(text: &str) -> i32 {
    let mut history = vec![0];
    let mut total = 0;
    for line in text.lines().cycle() {
        let value: i32 = line.parse().expect("Not a number.");
        total += value;
        if history.contains(&total) {
            return total;
        }
        history.push(total);
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::final_frequency;
    use super::first_duplicate_frequency;

    #[test]
    fn final_all_positives() {
        assert_eq!(3, final_frequency("+1\n+1\n+1"));
    }

    #[test]
    fn final_mixed_integers() {
        assert_eq!(0, final_frequency("+1\n+1\n-2"));
    }

    #[test]
    fn final_all_negatives() {
        assert_eq!(-6, final_frequency("-1\n-2\n-3"));
    }

    #[test]
    fn duplicate_simple() {
        assert_eq!(0, first_duplicate_frequency("+1\n-1"));
    }

    #[test]
    fn duplicate_ten() {
        assert_eq!(10, first_duplicate_frequency("+3\n+3\n+4\n-2\n-4"));
    }

    #[test]
    fn duplicate_five() {
        assert_eq!(5, first_duplicate_frequency("-6\n+3\n+8\n+5\n-6"));
    }

    #[test]
    fn duplicate_fourteen() {
        assert_eq!(14, first_duplicate_frequency("+7\n+7\n-2\n-7\n-4"));
    }
}
