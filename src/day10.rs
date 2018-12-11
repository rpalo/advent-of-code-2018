/// Day 10: The Stars Align
/// 
/// Figure out what message appears as 2D vectors line up

use regex::Regex;

/// A star in the sky that has position and velocity in 2 dimensions
struct Star {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Star {
    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

/// A night sky that has kinematic stars in it
pub struct Sky {
    stars: Vec<Star>,
}

impl Sky {
    pub fn new() -> Self {
        Self { stars: vec![] }
    }

    /// Loads stars in the sky from text
    pub fn from_text(text: &str) -> Self {
        let mut sky = Self::new();
        let number_regex = Regex::new(r"-?\d+").unwrap();
        for line in text.lines() {
            let mut numbers = number_regex.find_iter(line);
            let x = numbers.next().unwrap().as_str().parse().unwrap();
            let y = numbers.next().unwrap().as_str().parse().unwrap();
            let vx = numbers.next().unwrap().as_str().parse().unwrap();
            let vy = numbers.next().unwrap().as_str().parse().unwrap();
            sky.stars.push(Star { x, y, vx, vy });
        }
        sky
    }

    /// Updates the position of each star based on its velocity
    pub fn update(&mut self) {
        for star in self.stars.iter_mut() {
            star.update();
        }
    }

    /// Loop over time, updating the Sky and potentially displaying the
    /// current state.  Only displays if the stars are clustered enough
    /// together.  Stars are '#' and empty sky is '.'
    pub fn display(&mut self, frames: isize) {
        for t in 0..frames {
            self.update();

            let xmin = self.stars.iter().map(|star| star.x).min().unwrap();
            let xmax = self.stars.iter().map(|star| star.x).max().unwrap();
            let ymin = self.stars.iter().map(|star| star.y).min().unwrap();
            let ymax = self.stars.iter().map(|star| star.y).max().unwrap();

            // Only display if the bounds are reasonable
            if (ymax - ymin) > 30 {
                continue;
            }
            println!("");
            println!("t = {}", t);
            for y in (ymin - 5)..=(ymax + 5) {
                for x in (xmin - 5)..=(xmax + 5) {
                    if self.stars.iter().any(|star| star.x == x && star.y == y) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
        }
    }
}