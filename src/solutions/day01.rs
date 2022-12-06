pub fn part_one(input: &str) -> u32 {
    input.as_bytes()
        .iter()
        .chain(&[b'\n'; 1]) //keep from special casing last line
        .fold((0, 0, 0), |(max, sum, current), c| {
            if *c == b'\n' {
                if current == 0 {
                    (max.max(sum), 0, 0)
                }
                else {
                    (max, sum + current, 0)
                }
                
            } else {
                (max, sum, current * 10 + (*c - b'0') as u32)
            }
        }).0
}

fn track_top_3(top3: (u32, u32, u32), n: u32) -> (u32, u32, u32) {
    match (n, top3) {
        (_, (top, second, third)) if n < third => (top, second, third),
        (n, (top, second, _)) if n > top => (n, top, second),
        (n, (top, second, _)) if n > second => (top, n, second),
        (n, (top, second, _)) => (top, second, n),
    }
}

pub fn part_two(input: &str) -> u32 {
    let top3 = input.as_bytes()
        .iter()
        .chain(&[b'\n'; 2]) //keep from special casing last line
        .fold(((0, 0, 0), 0, 0), |(top3, sum, current), c| {
            if *c == b'\n' {
                if current == 0 {
                    (track_top_3(top3, sum), 0, 0)
                }
                else {
                    (top3, sum + current, 0)
                }
                
            } else {
                (top3, sum, current * 10 + (*c - b'0') as u32)
            }
        }).0;
        top3.0 + top3.1 + top3.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 24000);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 45000);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 1);
        assert_eq!(part_one(&input), 68775);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 1);
        assert_eq!(part_two(&input), 202585);
    }
}