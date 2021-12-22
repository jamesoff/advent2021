fn main() {
    println!("Hello, world!");
    println!("challenge1 is {}", challenge1());
    println!("challenge2 is {}", challenge2());
}

struct Grid {
    items: Vec<i32>,
    flashes: Vec<bool>,
    rows: usize,
    cols: usize,
    max: i32,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            items: Vec::new(),
            flashes: Vec::new(),
            rows: 0,
            cols: 0,
            max: 10,
        }
    }

    pub fn point(&self, row: usize, col: usize) -> i32 {
        let index = row * self.cols + col;
        // println!("index for row {} col {} = {}", row, col, index);
        self.items[index]
    }

    pub fn has_flashed(self, row: usize, col: usize) -> bool {
        self.flashes[row * self.cols + col]
    }

    pub fn set_point(&mut self, row: usize, col: usize, value: i32) {
        let index = row * self.cols + col;
        let old_value = self.items[index];
        if old_value < self.max && value == self.max {
            //self.flashes[index] = true;
        }
        self.items[index] = value;
    }

    pub fn init_flashes(&mut self) {
        self.flashes = vec![false; self.rows * self.cols];
    }

    pub fn add_row_from_str(&mut self, new_row: &str) {
        for item in new_row.chars() {
            match item.to_digit(10) {
                Some(x) => self.items.push(x.try_into().unwrap()),
                None => panic!("bad digit {} in line {}", item, new_row),
            }
        }
        self.rows += 1;
        if self.rows == 1 {
            self.cols = self.items.len();
            // println!("welcome to your new grid, cols = {}", self.cols);
        } else {
            if self.items.len() % self.cols != 0 {
                panic!("new row is not correct length");
            }
            // println!(
            // "grid size is rows {} cols {}, len {}",
            // self.rows,
            // self.cols,
            // self.items.len()
            // );
        }
    }

    pub fn load_from_multiline(&mut self, data: &str) {
        for line in data.split("\n") {
            self.add_row_from_str(line.trim());
        }
    }

    pub fn above(&self, row: usize, col: usize) -> Result<i32, String> {
        if row == 0 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row - 1, col))
    }

    pub fn left(&self, row: usize, col: usize) -> Result<i32, String> {
        if col == 0 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row, col - 1))
    }

    pub fn below(&self, row: usize, col: usize) -> Result<i32, String> {
        if row == self.rows - 1 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row + 1, col))
    }

    pub fn right(&self, row: usize, col: usize) -> Result<i32, String> {
        if col == self.cols - 1 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row, col + 1))
    }

    pub fn top_left(&self, row: usize, col: usize) -> Result<i32, String> {
        if col == 0 || row == 0 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row - 1, col - 1))
    }

    pub fn top_right(&self, row: usize, col: usize) -> Result<i32, String> {
        if col == self.cols - 1 || row == 0 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row - 1, col + 1))
    }

    pub fn bottom_left(&self, row: usize, col: usize) -> Result<i32, String> {
        if col == 0 || row == self.rows - 1 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row + 1, col - 1))
    }

    pub fn bottom_right(&self, row: usize, col: usize) -> Result<i32, String> {
        if col == self.cols - 1 || row == self.rows - 1 {
            return Err("out of bounds".to_string());
        }
        Ok(self.point(row + 1, col + 1))
    }

    pub fn is_low_point(&self, row: usize, col: usize) -> bool {
        let here = self.point(row, col);
        self.above(row, col).unwrap_or(11) > here
            && self.left(row, col).unwrap_or(11) > here
            && self.right(row, col).unwrap_or(11) > here
            && self.below(row, col).unwrap_or(11) > here
    }

    pub fn find_low_sum(&self) -> i32 {
        let mut total = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.is_low_point(row, col) {
                    let value = self.point(row, col);
                    // println!("low point at row {} col {} ({})", row, col, value);
                    total += value + 1;
                }
            }
        }
        total
    }

    pub fn increment_grid(&mut self) {
        self.items = self
            .items
            .iter()
            .map(|x| if *x < self.max { *x + 1 } else { *x })
            .collect();
        //self.flashes = self
        //    .items
        //    .iter()
        //    .map(|x| if *x == self.max { true } else { false })
        //    .collect();
        self.init_flashes();
    }

    pub fn handle_flash(&mut self, row: usize, col: usize) -> bool {
        let index = row * self.cols + col;

        if self.point(row, col) == self.max && !self.flashes[index] {
            // mark as handled
            self.flashes[index] = true;
            self.increment_top_left(row, col);
            self.increment_top(row, col);
            self.increment_top_right(row, col);
            self.increment_right(row, col);
            self.increment_bottom_right(row, col);
            self.increment_bottom(row, col);
            self.increment_bottom_left(row, col);
            self.increment_left(row, col);
            return true;
        }
        false
    }

    pub fn reset_grid(&mut self) {
        self.items = self
            .items
            .iter()
            .map(|x| if *x == self.max { 0 } else { *x })
            .collect();
        self.init_flashes();
    }

    pub fn increment_top_left(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        //println!("top left of row {} col {}", row, col);
        if let Ok(value) = self.top_left(row, col) {
            if value < limit {
                self.set_point(row - 1, col - 1, value + 1);
            } else if value == limit {
                self.set_point(row - 1, col - 1, value + 1);
                //self.flashes[(row - 1) * self.cols + col - 1] = true;
            }
        }
    }

    pub fn increment_top(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.above(row, col) {
            if value < limit {
                self.set_point(row - 1, col, value + 1);
            } else if value == limit {
                self.set_point(row - 1, col, value + 1);
                //self.flashes[(row - 1) * self.cols + col] = true;
            }
        }
    }

    pub fn increment_top_right(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.top_right(row, col) {
            if value < limit {
                self.set_point(row - 1, col + 1, value + 1);
            } else if value == limit {
                self.set_point(row - 1, col + 1, value + 1);
                //self.flashes[(row - 1) * self.cols + col + 1] = true;
            }
        }
    }

    pub fn increment_right(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.right(row, col) {
            if value < limit {
                self.set_point(row, col + 1, value + 1);
            } else if value == limit {
                self.set_point(row, col + 1, value + 1);
                //self.flashes[row * self.cols + col + 1] = true;
            }
        }
    }

    pub fn increment_bottom_right(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.bottom_right(row, col) {
            if value < limit {
                self.set_point(row + 1, col + 1, value + 1);
            } else if value == limit {
                self.set_point(row + 1, col + 1, value + 1);
                //self.flashes[(row + 1) * self.cols + col + 1] = true;
            }
        }
    }

    pub fn increment_bottom(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.below(row, col) {
            if value < limit {
                self.set_point(row + 1, col, value + 1);
            } else if value == limit {
                self.set_point(row + 1, col, value + 1);
                //self.flashes[(row + 1) * self.cols + col] = true;
            }
        }
    }

    pub fn increment_bottom_left(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.bottom_left(row, col) {
            if value < limit {
                self.set_point(row + 1, col - 1, value + 1);
            } else if value == limit {
                self.set_point(row + 1, col - 1, value + 1);
                //self.flashes[(row + 1) * self.cols + col - 1] = true;
            }
        }
    }

    pub fn increment_left(&mut self, row: usize, col: usize) {
        let limit = self.max - 1;
        if let Ok(value) = self.left(row, col) {
            //println!("flash at {} {} ({})", row, col, value);
            if value < limit {
                self.set_point(row, col - 1, value + 1);
            } else if value == limit {
                self.set_point(row, col - 1, value + 1);
                //self.flashes[row * self.cols + col - 1] = true;
            }
        }
    }

    pub fn process_all_flashes(&mut self) -> i32 {
        let mut again = true;
        while again {
            //println!("processing");
            again = false;
            for row in 0..self.rows {
                for col in 0..self.cols {
                    let result = self.handle_flash(row, col);
                    if result {
                        again = true;
                    }
                }
            }
            //println!("after process: {:?}", self.items);
        }
        let flash_count = self
            .flashes
            .iter()
            .map(|x| if *x { 1 } else { 0 })
            .sum::<i32>();
        //println!("done processing, {} flashes", flash_count);
        flash_count
    }

    pub fn step(&mut self) -> i32 {
        self.increment_grid();
        let flash_count = self.process_all_flashes();
        self.reset_grid();
        flash_count
    }

    pub fn step_all_flash(&mut self) -> bool {
        self.step();
        if self.items.iter().sum::<i32>() == 0 {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut g = Grid::new();
        g.add_row_from_str("2199943210");
        assert_eq!(g.rows, 1);
        assert_eq!(g.cols, 10);
        assert_eq!(g.point(0, 0), 2);
        assert_eq!(g.point(0, 9), 0);
        g.add_row_from_str("3987894921");
        assert_eq!(g.rows, 2);
        assert_eq!(g.point(1, 0), 3);
        assert_eq!(g.point(1, 5), 9);
    }

    #[test]
    fn test_load() {
        let mut g = Grid::new();
        g.load_from_multiline(
            "2199943210
            3987894921
            9856789892
            8767896789
            9899965678",
        );
        assert_eq!(g.rows, 5);
        assert_eq!(g.cols, 10);
        assert_eq!(g.point(4, 9), 8);
        assert_eq!(g.above(1, 0), Ok(2));
        assert_eq!(g.below(1, 0), Ok(9));
        assert_eq!(g.above(0, 0), Err("out of bounds".to_string()));
        assert_eq!(g.right(1, 2), Ok(7));
        assert_eq!(g.left(1, 2), Ok(9));
        assert_eq!(g.top_left(1, 1), Ok(2));
        assert_eq!(g.top_right(1, 1), Ok(9));
        assert_eq!(g.bottom_left(1, 1), Ok(9));
        assert_eq!(g.bottom_right(1, 1), Ok(5));
        assert_eq!(g.top_left(0, 0), Err("out of bounds".to_string()));
        assert_eq!(g.top_right(0, 9), Err("out of bounds".to_string()));
        assert_eq!(g.bottom_left(4, 0), Err("out of bounds".to_string()));
        assert_eq!(g.bottom_right(4, 9), Err("out of bounds".to_string()));

        assert_eq!(g.is_low_point(0, 1), true);
        assert_eq!(g.is_low_point(0, 9), true);

        assert_eq!(g.find_low_sum(), 15);
    }

    #[test]
    fn test_set() {
        let mut g = Grid::new();
        g.add_row_from_str("00000");
        g.add_row_from_str("00000");
        g.set_point(0, 3, 1);
        assert_eq!(g.point(0, 3), 1);
    }

    #[test]
    fn test_increment_grid() {
        let mut g = Grid::new();
        g.add_row_from_str("00000");
        g.add_row_from_str("12345");
        g.add_row_from_str("67890");
        g.increment_grid();
        assert_eq!(g.point(0, 0), 1);
        assert_eq!(g.point(1, 0), 2);
        assert_eq!(g.point(2, 2), 9);
        assert_eq!(g.point(2, 3), 10);
        assert_eq!(
            g.flashes,
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false
            ]
        );
    }

    #[test]
    fn test_increment_point() {
        let mut g = Grid::new();
        g.add_row_from_str("00000");
        g.add_row_from_str("12345");
        g.add_row_from_str("67890");
        g.increment_grid();
        g.handle_flash(1, 1);
        assert_eq!(g.point(1, 1), 3);
        assert_eq!(g.point(0, 0), 1);
        g.handle_flash(2, 3);
        println!("{:?}", g.items);
        println!("{:?}", g.flashes);
        assert_eq!(g.point(2, 3), 10);
        assert_eq!(g.point(1, 2), 5);
        assert_eq!(
            g.flashes,
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, true, false
            ]
        );
    }

    #[test]
    fn test_reset_grid() {
        let mut g = Grid::new();
        g.add_row_from_str("00000");
        g.add_row_from_str("12345");
        g.add_row_from_str("67890");
        g.increment_grid();
        g.reset_grid();
        assert_eq!(g.items, vec![1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]);
    }

    #[test]
    fn test_sample() {
        let mut g = Grid::new();
        let mut flashes = 0;
        g.load_from_multiline(
            "11111
            19991
            19191
            19991
            11111",
        );
        flashes += g.step();
        assert_eq!(
            g.items,
            vec![3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3]
        );
        flashes += g.step();
        assert_eq!(
            g.items,
            vec![4, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4, 5, 6, 5, 4]
        );
        assert_eq!(flashes, 9);
    }

    #[test]
    fn test_larger() {
        let mut g = Grid::new();
        let mut flashes = 0;
        g.load_from_multiline(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );
        for _ in 0..100 {
            println!("{:?}", g.items);
            flashes += g.step();
        }
        assert_eq!(flashes, 1656);
    }
}

fn challenge1() -> i32 {
    let mut g = Grid::new();
    let mut flashes = 0;
    g.load_from_multiline(
        "7313511551
3724855867
2374331571
4438213437
6511566287
6727245532
3736868662
2348138263
2417483121
8812617112",
    );
    for _ in 0..100 {
        flashes += g.step();
    }
    flashes
}

fn challenge2() -> i32 {
    let mut g = Grid::new();
    g.load_from_multiline(
        "7313511551
3724855867
2374331571
4438213437
6511566287
6727245532
3736868662
2348138263
2417483121
8812617112",
    );
    let mut step = 1;
    while !g.step_all_flash() {
        step += 1;
    }
    step
}
