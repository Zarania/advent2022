fn char_to_int(character: u8) -> u8 {
    match character {
        b'a'..=b'z' => character - b'a' + 1,
        _ => character - b'A' + 27
    }
}

pub fn part_one(input: &str) -> u32 {
    input.lines()
        .map(|line| 
            (line[..(line.len()/2)].chars().map(|c| char_to_int(c as u8)).fold(0, |map, val| map | 1u64 << val) &
            line[(line.len()/2)..].chars().map(|c| char_to_int(c as u8)).fold(0, |map, val| map | 1u64 << val))
            .trailing_zeros()
        )
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines()
        .map(|line| 
            line.chars().map(|c| char_to_int(c as u8)).fold(0, |map, val| map | 1u64 << val
        ))
        .array_chunks::<3>()
        .map(|m| (m[0] & m[1] & m[2]).trailing_zeros())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 157);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 70);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 3);
        assert_eq!(part_one(&input), 8349);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 3);
        assert_eq!(part_two(&input), 2681);
    }
}