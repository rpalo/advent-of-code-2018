/// Day 11: Chronal Charge
/// 
/// Figure out the power contained in power cells

pub struct Grid {
    cells: [[i16; 300]; 300],
    serial: isize,
}

impl Grid {
    pub fn new(serial: isize) -> Self {
        let mut grid = Self { serial, cells: [[0; 300]; 300] };
        grid.generate_values();
        grid
    }

    fn generate_values(&mut self) {
        for (i, row) in self.cells.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value = Grid::power_level(self.serial, (i + 1) as i16, (j + 1) as i16);
            }
        }
    }

    fn power_level(serial: isize, x: i16, y: i16) -> i16 {
        let mut result = (x as isize + 10) * (y as isize) + serial;
        result *= x as isize + 10;
        result = (result / 100) % 10;
        (result - 5) as i16
    }

    pub fn best_cell(&self) -> (usize, usize) {
        let mut max_value = 0;
        let mut max_location = (0, 0);
        for i in 0..298 {
            for j in 0..298 {
                let value: i16 = self.cells.iter()
                    .skip(i).take(3)
                    .flat_map(|row| row.iter().skip(j).take(3) )
                    .sum();
                if value > max_value {
                    max_value = value;
                    max_location = (i + 1, j + 1);
                }
            }
        }
        max_location
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let grid = Grid::new(18);
        assert_eq!(grid.best_cell(), (33, 45));
    }

    #[test]
    fn test_part_one_2() {
        let grid = Grid::new(42);
        assert_eq!(grid.best_cell(), (21, 61));
    }

    #[test]
    fn test_power_level_57() {
        assert_eq!(-5, Grid::power_level(57, 122, 79));
    }

    #[test]
    fn test_power_level_39() {
        assert_eq!(0, Grid::power_level(39, 217, 196));
    }

    #[test]
    fn test_power_level_71() {
        assert_eq!(4, Grid::power_level(71, 101, 153));
    }

}