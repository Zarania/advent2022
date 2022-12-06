use std::cmp;

pub fn part_one(input: &str) -> u32 {
    let mut parts: [i32; 4] = [0, 0, 0, 0];
    let mut n = 0;
    input
        .as_bytes()
        .iter()
        .chain(&[b'\n'; 1]) //keep from special casing last line
        .fold(0, |total: u32, byte| match byte {
            b'-' | b',' => {
                n += 1;
                total
            }
            b'\n' => {
                n = 0;
                let result = ((parts[0] - parts[2]) * (parts[1] - parts[3]) <= 0) as u32;
                parts = [0, 0, 0, 0];
                total + result
            }
            _ => {
                parts[n] = parts[n] * 10 + (byte - b'0') as i32;
                total
            }
        })
}

pub fn part_two(input: &str) -> u32 {
    let mut parts: [i32; 4] = [0, 0, 0, 0];
    let mut n = 0;
    input
        .as_bytes()
        .iter()
        .chain(&[b'\n'; 1]) //keep from special casing last line
        .fold(0, |total: u32, byte| match byte {
            b'-' | b',' => {
                n += 1;
                total
            }
            b'\n' => {
                n = 0;
                let result = (cmp::max(parts[0], parts[2]) <= cmp::min(parts[1], parts[3])) as u32;
                parts = [0, 0, 0, 0];
                total + result
            }
            _ => {
                parts[n] = parts[n] * 10 + (byte - b'0') as i32;
                total
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 4);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 4);
        assert_eq!(part_one(&input), 588);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 4);
        assert_eq!(part_two(&input), 911);
    }
}
