use std::time::Instant;

const DROP_X: usize = 500;
const X_MIN: usize = 0;
const X_MAX: usize = DROP_X * 2;
const WIDTH: usize = X_MAX - X_MIN + 1;

struct Map {
    grid: Vec<bool>,
    path: Vec<usize>,
    height: usize
}

impl Map {
    fn drop_grain(&mut self) -> bool {
        if self.path.is_empty() {
            panic!();
        }

        loop {
            let position = *self.path.last().unwrap();
            if position / WIDTH >= self.height - 1 {
                return false;
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

    fn drop_grain_2(&mut self) -> bool {
        if self.path.is_empty() {
            return false;
        }

        loop {
            let position = *self.path.last().unwrap();
            if position / WIDTH >= self.height - 1 {
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
        let mut height = 0;
        let positions = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .filter_map(|s| {
                        s.split_once(',').map(|(a, b)| {
                            let coords = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
                            height = height.max(coords.1);
                            coords
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
            height = height + 2;
            let mut grid = vec![false; X_MAX * height];
            positions.iter().for_each(|line| {
                for pair in line.windows(2) {
                    let (start, end) = (pair[0], pair[1]);
                    //left to right
                    if start.1 == end.1 {
                        let (min, max) = (start.0.min(end.0), start.0.max(end.0));
                        for i in min..=max {
                            grid[i + start.1 * X_MAX] = true;
                        }
                    } else {
                        let (min, max) = (start.1.min(end.1), start.1.max(end.1));
                        for i in min..=max {
                            grid[start.0 + i * X_MAX] = true;
                        }
                    }
                }
            });
        let mut path = Vec::with_capacity(height);
        path.push(DROP_X);
        Map { grid, path,  height}
    }

    fn _print_grid(&self, height: usize, width: usize, start: usize) {
        for y in 0..height {
            for x in start..=start + width {
                if self.grid[x + y * X_MAX] {
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

    #[test]
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
    }
}
