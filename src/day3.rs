
use regex::Regex;
use std::collections::HashMap;

/// An X, Y grid of Santa's fabric that elves can lay claim to
struct Fabric {
    squares: HashMap<(usize, usize), usize>,
}

struct Claim {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Fabric {
    fn new() -> Self {
        Self { squares: HashMap::new() }
    }

    /// Increments the amount of claims covering each of the cells inside
    /// the rectangle.
    fn claim(&mut self, claim: &Claim) {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                if x > 999 || y > 999 {
                    continue;
                }
                *self.squares.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    /// Counts how many cells have more than one claim on them
    fn count_conflicts(&self) -> usize {
        self.squares.values().filter(|count| **count >= 2).count()
    }

    fn total_squares(&self) -> usize {
        self.squares.iter().count()
    }
}



/// Processes a claim string into an actual Claim
/// 
/// claim string pattern is #<id> @ <left>,<top>: <width>x<height>
/// Since all the numbers are disjoint, we can just match all the 
/// separated numbers in order.
fn process_claim(claim_text: &str) -> Claim {
    lazy_static! {
        static ref claim_re: Regex = Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    }
    let claim_parts = claim_re.captures(claim_text).unwrap();
    Claim {
        left: claim_parts["left"].parse().unwrap(),
        top: claim_parts["top"].parse().unwrap(),
        width: claim_parts["width"].parse().unwrap(),
        height: claim_parts["height"].parse().unwrap(),
    }
}

pub fn count_conflicting_squares(text: &str) -> usize {
    let mut fabric = Fabric::new();

    for line in text.lines() {
        let claim = process_claim(line);
        fabric.claim(&claim);
    }

    fabric.count_conflicts()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let claim_text = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

        assert_eq!(4, count_conflicting_squares(claim_text));
    }

    #[test]
    fn test_overlapping_width_by_one() {
        let claim_text = "#1 @ 0,0: 5x1
#2 @ 4,0: 2x1";

        assert_eq!(1, count_conflicting_squares(claim_text));
    }

    #[test]
    fn test_adjacent_x_doesnt_overlap() {
        let claim_text = "#1 @ 0,0: 5x1
#2 @ 5,0: 2x1";

        assert_eq!(0, count_conflicting_squares(claim_text));
    }

    #[test]
    fn test_y_overlaps_by_one() {
        let claim_text = "#1 @ 0,0: 2x5
#2 @ 0,4: 2x2";

        assert_eq!(2, count_conflicting_squares(claim_text));
    }

    #[test]
    fn test_adjacent_y_doesnt_overlap() {
        let claim_text = "#1 @ 0,0: 2x5
#2 @ 0,5: 2x2";

        assert_eq!(0, count_conflicting_squares(claim_text));
    }

    #[test]
    fn test_fabric_has_right_number_of_squares() {
        let mut fabric = Fabric::new();
        let claim = Claim { left: 3, top: 3, width: 5, height: 4 };
        fabric.claim(&claim);
        assert_eq!(20, fabric.total_squares());
    }
}