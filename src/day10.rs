/// Day 10: The Stars Align
/// 
/// Figure out what message appears as 2D vectors line up

use regex::Regex;

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

pub struct Sky {
    stars: Vec<Star>,
}

impl Sky {
    pub fn new() -> Self {
        Self { stars: vec![] }
    }

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

    pub fn update(&mut self) {
        for star in self.stars.iter_mut() {
            star.update();
        }
    }

    pub fn display(&self) {
        let xmin = self.stars.iter().map(|star| star.x).min().unwrap();
        let xmax = self.stars.iter().map(|star| star.x).max().unwrap();
        let ymin = self.stars.iter().map(|star| star.y).min().unwrap();
        let ymax = self.stars.iter().map(|star| star.y).max().unwrap();
        if (ymax - ymin) > 30 {
            return;
        }
        println!("");
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