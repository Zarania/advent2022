use hashers::fx_hash::FxHasher;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;

use crate::int_from_bytes_exact;

fn calculate<const COUNT: usize>(input: &str) -> u32 {
    let mut knots = [(0, 0); COUNT];

    let mut size = 0;
    let moves = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let amount = int_from_bytes_exact::<u32>(&line[2..]);
            size += amount;

            (line[0], amount)
        })
        .collect::<Vec<_>>();
    let mut positions =
        HashSet::with_capacity_and_hasher(size as usize, BuildHasherDefault::<FxHasher>::default());
    positions.insert(knots[COUNT - 1]);

    for (direction, amount) in moves {
        for _ in 0..amount {
            match direction {
                b'R' => knots[0].0 += 1,
                b'L' => knots[0].0 -= 1,
                b'U' => knots[0].1 += 1,
                b'D' => knots[0].1 -= 1,
                _ => unreachable!(),
            }
            for i in 0..knots.len() - 1 {
                let x_dist: i32 = knots[i].0 - knots[i + 1].0;
                let y_dist: i32 = knots[i].1 - knots[i + 1].1;
                if x_dist.abs() >= 2 || y_dist.abs() >= 2 {
                    knots[i + 1] = (
                        knots[i + 1].0 + x_dist.signum(),
                        knots[i + 1].1 + y_dist.signum(),
                    );
                } else {
                    break;
                }
            }
            positions.insert(knots[COUNT - 1]);
        }
    }

    positions.len() as u32
}

pub fn part_one(input: &str) -> u32 {
    calculate::<2>(input)
}

pub fn part_two(input: &str) -> u32 {
    calculate::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 9);
        assert_eq!(part_one(&input), 6503);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 9);
        assert_eq!(part_two(&input), 2724);
    }
}
