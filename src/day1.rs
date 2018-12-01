pub fn final_frequency(text: &str) -> i32 {
    text.lines()
        .map(|value: &str| -> i32 {value.parse().expect("Not a number")})
        .sum()
}

#[cfg(test)]
mod tests {
    use super::final_frequency;

    #[test]
    fn all_positives() {
        assert_eq!(3, final_frequency("+1\n+1\n+1"));
    }

    #[test]
    fn mixed_integers() {
        assert_eq!(0, final_frequency("+1\n+1\n-2"));
    }

    #[test]
    fn all_negatives() {
        assert_eq!(-6, final_frequency("-1\n-2\n-3"));
    }
}
