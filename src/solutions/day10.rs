use crate::int_from_bytes_signed;

pub fn part_one(input: &str) -> i32 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        //note cycle starts at 2 due to how we calculate
        .fold((0, 1, 2), |(total, register, cycle), line| {
            if line[0] == b'n' {
                //noop
                //(cycle + 20) % 40 == 0 as i32 gives us 20 / 40 * 1, otherwise * 0 to only increment at the correct time
                (
                    total + (register * cycle) * ((cycle + 20) % 40 == 0) as i32,
                    register,
                    cycle + 1,
                )
            } else {
                //add
                //to remove some branching, do some  terrible things
                //by doing a mod 40 on cycle + 21 and checking if this is < 2 this gives us 19 / 20, 59 / 60, etc
                let modulo = (cycle + 21) % 40;
                let num = int_from_bytes_signed::<i32>(&line[5..]);
                //again, to remove branching we cast from bool to i32 and if false that gives a 0 to remove that piece
                //the register should only increase if we are on the % 40 numbers, so check mod < 2
                // cycle + 1 - modulo gives us 20 / 60 instead of 19 / 20 / 59 / 60 etc
                // num * modulo == 0 because if on a 19 / 59 we finish the add then multiply by cycle
                (
                    total
                        + ((register + (num * (modulo == 0) as i32))
                            * (cycle + 1 - modulo)
                            * (modulo < 2) as i32),
                    register + num,
                    cycle + 2,
                )
            }
        })
        .0
}

pub fn part_two(input: &str) -> String {
    let mut register: i32 = 1;
    let mut cycle = 0;
    let bytes = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .flat_map(|line| {
            if line[0] == b'n' {
                //similar trick here by multiplying 11 by the result of abs_diff to change between # and .
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

    //returning the string to make unit testing easier
    //if you want to actually print the answer on a 40 line grid, uncomment this part
    /*for b in bytes.chunks(40).map(|b| {
        let line = b
            .iter()
            .map(|&c| match c {
                b'#' => 9619,
                b'.' => 9617,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        String::from_utf16(&line).unwrap()
    }) {
        println!("{}", b);
    }
    println!();*/

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
