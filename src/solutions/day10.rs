use crate::int_from_bytes_signed;

pub fn part_one(input: &str) -> i32 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .fold((0, 1, 2), |(total, register, cycle), line| {
            if line[0] == b'n' {
                (total + (register * cycle) * ((cycle + 20) % 40 == 0) as i32, register, cycle + 1)
            } else {
                let modulo = (cycle + 21) % 40;
                let num = int_from_bytes_signed::<i32>(&line[5..]);
                (
                    total + ((register + (num * (modulo == 0) as i32)) * (cycle + 1 - modulo) * (modulo < 2) as i32),
                    register + num,
                    cycle + 2,
                )
            }
        })
        .0
}

pub fn part_two(input: &str) -> String {
    let mut register : i32 = 1;
    let mut cycle = 0;
    let bytes = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .flat_map(|line| {
            if line[0] == b'n' {
                let diff = register.abs_diff(cycle) > 1;
                cycle = (cycle + 1) % 40;
                vec![b'#' + 11 * diff as u8]
            } else {
                let mut result = Vec::with_capacity(2);
                result.push(b'#' + 11 * (register.abs_diff(cycle) > 1) as u8);

                cycle = (cycle + 1) % 40;
                result.push(b'#' + 11 * (register.abs_diff(cycle) > 1) as u8);

                cycle = (cycle + 1) % 40;

                register += int_from_bytes_signed::<i32>(&line[5..]);
                result
            }
        })
        .collect::<Vec<_>>();

    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 10);
        assert_eq!(part_one(&input), 13140);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 10);
        assert_eq!(part_two(&input), "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".to_string());
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 10);
        assert_eq!(part_one(&input), 14560);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 10);
        assert_eq!(part_two(&input), "####.#..#.###..#..#.####.###..#..#.####.#....#.#..#..#.#..#.#....#..#.#..#....#.###..##...#..#.####.###..#..#.#..#...#..#....#.#..###..#..#.#....###..#..#..#...#....#.#..#.#..#..#.#....#....#..#.#....####.#..#.#..#.#..#.####.#.....##..####.".to_string());
    }
}
