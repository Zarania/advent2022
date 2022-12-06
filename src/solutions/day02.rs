pub fn part_one(input: &str) -> u32 {
    let results = [4, 8, 3, 1, 5, 9, 7, 2, 6];
    input.as_bytes().chunks(4)
        .map(|line|
            results[((line[0] - b'A') as usize) * 3 + (line[2] - b'X') as usize]
        )
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let results = [3, 4, 8, 1, 5, 9, 2, 6, 7];
    input.as_bytes().chunks(4)
        .map(|line|
            results[((line[0] - b'A') as usize) * 3 + (line[2] - b'X') as usize]
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 12);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 2);
        assert_eq!(part_one(&input), 12772);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 2);
        assert_eq!(part_two(&input), 11618);
    }
}