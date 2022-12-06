pub fn part_one(_input: &str) -> u32 {
    0
}

pub fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 0);
    }
}
