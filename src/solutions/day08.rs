fn get_byte(x: usize, y: usize, length: usize, slice: &mut [i8]) -> &mut i8 {
    &mut slice[(y * length) + x]
}

pub fn part_one(input: &str) -> u32 {
    let mut forest: Vec<_> = input.as_bytes().iter().map(|&b| b as i8).collect();
    let length = forest
        .iter()
        .enumerate()
        .find(|(_, &b)| b == b'\n' as i8)
        .unwrap()
        .0
        + 1;

    let mut count = 0;
    for i in 0..length - 1 {
        let mut west_max = -1i8;
        let mut west_max_index = 0;

        for j in 0..length - 1 {
            let byte = get_byte(j, i, length, &mut forest);
            let west_value = *byte & 15;
            if west_max < west_value {
                count += 1;
                west_max = west_value;
                *byte |= i8::MIN;
                west_max_index = j;
            }
        }

        let mut east_max = -1i8;
        for j in (west_max_index + 1..length - 1).rev() {
            let byte = get_byte(j, i, length, &mut forest);
            let east_value = *byte & 15;
            if east_max < east_value {
                count += 1;
                east_max = east_value;
                *byte |= i8::MIN;
            }
        }
    }

    for i in 0..length - 1 {
        let mut north_max = -1i8;
        let mut north_max_index = 0;

        for j in 0..length - 1 {
            let byte = get_byte(i, j, length, &mut forest);
            let north_value = *byte & 15;
            if north_max < north_value {
                count += (*byte >= 0) as u32;
                north_max = north_value;
                north_max_index = j;
            }
        }

        let mut south_max = -1i8;
        for j in (north_max_index + 1..length - 1).rev() {
            let byte = get_byte(i, j, length, &mut forest);
            let south_value = *byte & 15;
            if south_max < south_value {
                count += (*byte >= 0) as u32;
                south_max = south_value;
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> u32 {
    let forest: Vec<&[u8]> = input.as_bytes().split(|&b| b == b'\n').collect();

    let length = forest.len();
    (1..length-1).map(|i| {
        (1..length - 1).map(|j| {
            let current = forest[i][j];
            
            forest[i][0..j]
                .iter()
                .rev()
                .position(|&b| b >= current)
                .unwrap_or(j.wrapping_sub(1))
                .wrapping_add(1)
            * (0..i)
                .rev()
                .position(|p| forest[p][j] >= current)
                .unwrap_or(i.wrapping_sub(1))
                .wrapping_add(1)
            * forest[i][j + 1..length]
                .iter()
                .position(|&b| b >= current)
                .unwrap_or(length.wrapping_sub(j).wrapping_sub(2))
                .wrapping_add(1)
            * (i + 1..length)
                .position(|p| forest[p][j] >= current)
                .unwrap_or(length.wrapping_sub(i).wrapping_sub(2))
                .wrapping_add(1)
        }).max().unwrap()
    }).max().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 8);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 8);
        assert_eq!(part_one(&input), 1672);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 8);
        assert_eq!(part_two(&input), 327180);
    }
}
