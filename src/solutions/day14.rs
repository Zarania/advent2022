const DROP_X: usize = 500;
const X_MIN: usize = 0;
const X_MAX: usize = DROP_X * 2;
const WIDTH: usize = X_MAX - X_MIN + 1;
//glancing at a few solutions, it appears 157 is the max height in all of them
//if your input has a different max height, you will need to change this
const HEIGHT: usize = 157 + 2;

struct Map {
    grid: [bool; X_MAX * HEIGHT],
    path: Vec<usize>,
}

impl Map {
    fn drop_grain(&mut self) -> bool {
        if self.path.is_empty() {
            panic!();
        }

        loop {
            let position = *self.path.last().unwrap();
            if position / WIDTH >= HEIGHT - 1 {
                return false;
            }
            let mut new_position = position;
            new_position += WIDTH;
            if !self.grid[new_position] {
                self.path.push(new_position);
                continue;
            }

            new_position -= 1;
            if !self.grid[new_position] {
                self.path.push(new_position);
                continue;
            }

            new_position += 2;
            if !self.grid[new_position] {
                self.path.push(new_position);
                continue;
            }
            self.grid[position] = true;
            self.path.pop();
            return true;
        }
    }

    fn drop_grain_2(&mut self) -> bool {
        if self.path.is_empty() {
            return false;
        }

        loop {
            let position = *self.path.last().unwrap();
            if position / WIDTH >= HEIGHT - 1 {
                self.grid[position] = true;
                self.path.pop();
                return true;
            }
            let mut new_position = position;
            new_position += X_MAX;
            if !self.grid[new_position] {
                self.path.push(new_position);
                continue;
            }

            new_position -= 1;
            if !self.grid[new_position] {
                self.path.push(new_position);
                continue;
            }

            new_position += 2;
            if !self.grid[new_position] {
                self.path.push(new_position);
                continue;
            }
            self.grid[position] = true;
            self.path.pop();
            return true;
        }
    }

    
    fn new(input: &str) -> Self {
        let mut has_prev = false;
        let mut is_y = false;
        let mut prev = 0;
        let mut current = 0;
        let mut sum = 0;
        let mut grid = [false; X_MAX * HEIGHT];
        
        fn draw_line(grid: &mut [bool; X_MAX * HEIGHT], from: usize, to: usize) {
            //same Y
            if from % WIDTH == to % WIDTH {
                let start = from.min(to);
                for y in 0..(from.abs_diff(to) / WIDTH) {
                    grid[start + y * WIDTH] = true;
                }
            } else {
                let start = from.min(to);
                for x in 0..=from.abs_diff(to) {
                    grid[start + x] = true;
                }
            }
        }

        input.as_bytes().iter().for_each(|byte| {
            match byte {
                //
                b' ' => if is_y {
                    let next = current + sum * WIDTH;
                    if has_prev {
                        draw_line(&mut grid, prev, next);
                    }
                    prev = next;
                    sum = 0;
                    is_y = false;
                    has_prev = true;
                }
                b',' => {
                    current = sum;
                    sum = 0;
                    is_y = true;
                }
                b'\n' => {
                    let next = current + sum * WIDTH;
                    draw_line(&mut grid, prev, next);
                    has_prev = false;
                    sum = 0;
                    is_y = false;
                }
                b'0'..=b'9' => {
                    sum = sum * 10 + (byte - b'0') as usize;
                }
                _ => {}
            }
        });

        let next = current + sum * WIDTH;
        draw_line(&mut grid, prev, next);
        
        let mut path = Vec::with_capacity(HEIGHT);
        path.push(DROP_X);
        Map { grid, path}
    }

    fn _print_grid(&self, height: usize, width: usize, start: usize) {
        for y in 0..height {
            for x in start..=start + width {
                if self.grid[x + y * WIDTH] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut grid = Map::new(input);

    let mut grains = 0;
    while grid.drop_grain() {
        grains += 1;
    }

    grains
}

pub fn part_two(input: &str) -> u32 {
    let mut grid = Map::new(input);
    let mut grains = 0;

    while grid.drop_grain_2() {
        grains += 1;
    }

    grains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_one(&input), 24);
    }

    /*#[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_two(&input), 93);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 14);
        assert_eq!(part_one(&input), 719);
    }

    #[test]
    fn solutions_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 14);
        assert_eq!(part_two(&input), 23390);
    }*/
}
