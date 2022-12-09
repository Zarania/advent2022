pub fn part_one(input: &str) -> u32 {
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|&(_, w)| {
            w[0] != w[1]
                && w[0] != w[2]
                && w[0] != w[3]
                && w[1] != w[2]
                && w[1] != w[3]
                && w[2] != w[3]
        })
        .unwrap()
        .0 as u32
        + 4
}

pub fn part_two(input: &str) -> u32 {
    let bytes = input.as_bytes();

    let mut total = 0u32;
    //Size 27 array for 26 characters to make it so we don't have to index -1
    let mut characters = [0; 27];
    for byte in bytes.iter().take(14) {
        //ascii a = 97 or 1100001. Use binary 31 or 11111 to easily get the 5 least significant digits so a = 1 and z = 26
        let index = *byte as usize & 31;
        characters[index] += 1;
        total += (characters[index] == 1) as u32;
    }

    if total == 14 {
        return 15;
    }

    for i in 14..bytes.len() {
        let previous = bytes[i - 14] as usize & 31;
        let next = bytes[i] as usize & 31;

        characters[previous] -= 1;
        total -= (characters[previous] == 0) as u32;

        characters[next] += 1;
        total += (characters[next] == 1) as u32;

        if total == 14 {
            return i as u32 + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 7);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 19);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 6);
        assert_eq!(part_one(&input), 1702);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 6);
        assert_eq!(part_two(&input), 3559);
    }
}
