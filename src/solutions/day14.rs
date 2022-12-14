use hashers::fx_hash::FxHasher;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}
struct Map {
    grid: HashSet<Coordinate, BuildHasherDefault<FxHasher>>,
    path: Vec<Coordinate>,
    height: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

impl Map {
    fn drop_grain(&mut self) -> bool {
        if self.path.is_empty() {
            panic!();
        }

        loop {
            let position = *self.path.last().unwrap();
            if position.y >= self.height {
                return false;
            }
            let mut new_position = position;
            new_position.y += 1;
            if !self.grid.contains(&new_position) {
                self.path.push(new_position);
                continue;
            }

            new_position.x -= 1;
            if !self.grid.contains(&new_position) {
                self.path.push(new_position);
                continue;
            }

            new_position.x += 2;
            if !self.grid.contains(&new_position) {
                self.path.push(new_position);
                continue;
            }
            self.grid.insert(position);
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
            if position.y > self.height {
                self.grid.insert(position);
                self.path.pop();
                return true;
            }
            let mut new_position = position;
            new_position.y += 1;
            if !self.grid.contains(&new_position) {
                self.path.push(new_position);
                continue;
            }

            new_position.x -= 1;
            if !self.grid.contains(&new_position) {
                self.path.push(new_position);
                continue;
            }

            new_position.x += 2;
            if !self.grid.contains(&new_position) {
                self.path.push(new_position);
                continue;
            }
            self.grid.insert(position);
            self.path.pop();
            return true;
        }
    }

    fn new(input: &str) -> Self {
        let mut grid = HashSet::with_capacity_and_hasher(
            1000,
            BuildHasherDefault::<FxHasher>::default(),
        );
        input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .filter_map(|s| {
                        s.split_once(',').map(|(a, b)| {
                            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .for_each(|line| {
                for pair in line.windows(2) {
                    let (start, end) = (pair[0], pair[1]);
                    //left to right
                    if start.1 == end.1 {
                        let (min, max) = (start.0.min(end.0), start.0.max(end.0));
                        for i in min..=max {
                            grid.insert(Coordinate::new(i, start.1));
                        }
                    } else {
                        let (min, max) = (start.1.min(end.1), start.1.max(end.1));
                        for i in min..=max {
                            grid.insert(Coordinate::new(start.0, i));
                        }
                    }
                }
            });
        let height = grid.iter().max_by(|c1, c2| c1.y.cmp(&c2.y)).unwrap().y;
        let mut path = Vec::with_capacity(height);
        path.push(Coordinate::new(500, 0));
        Map { grid, path, height }
    }

    fn _print_grid(&self) {
        let min_x = self.grid.iter().min_by(|c1, c2| c1.x.cmp(&c2.x)).unwrap().x;
        let max_x = self.grid.iter().max_by(|c1, c2| c1.x.cmp(&c2.x)).unwrap().x;

        for y in 0..=self.height {
            for x in min_x..=max_x {
                if self.grid.contains(&Coordinate::new(x, y)) {
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
