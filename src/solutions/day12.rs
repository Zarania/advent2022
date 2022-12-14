use std::collections::VecDeque;

fn search(
    map: &Vec<u8>,
    line_length: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut visited = vec![false; map.len()];
    let mut queue = VecDeque::with_capacity(64);
    queue.push_back((start, 0));
    while let Some(((x, y), len)) = queue.pop_front() {
        if (x, y) == end {
            return Some(len);
        }
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if nx > map.len() || ny > map.len() || nx + ny * line_length >= map.len() {
                continue;
            }

            let square = map[nx + ny * line_length];
            if (map[x + y * line_length] + 1 >= square) && !visited[nx + ny * line_length] {
                visited[nx + ny * line_length] = true;
                queue.push_back(((nx, ny), len + 1));
            }
        }
    }

    None
}

fn search_rev(map: &Vec<u8>, line_length: usize, start: (usize, usize), end: u8) -> Option<u32> {
    let mut visited = vec![false; map.len()];

    let mut queue = VecDeque::with_capacity(64);
    queue.push_back((start, 0));
    while let Some(((x, y), len)) = queue.pop_front() {
        if map[x + y * line_length] == end {
            return Some(len);
        }
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if nx > map.len() || ny > map.len() || nx + ny * line_length >= map.len() {
                continue;
            }

            let square = map[nx + ny * line_length];
            if (map[x + y * line_length] - 1 <= square) && !visited[nx + ny * line_length] {
                visited[nx + ny * line_length] = true;
                queue.push_back(((nx, ny), len + 1));
            }
        }
    }

    None
}

fn parse(input: &str) -> (Vec<u8>, usize, (usize, usize), (usize, usize)) {
    let mut map: Vec<_> = input.bytes().collect();
    let start = map.iter().position(|&c| c == b'S').unwrap();
    let line_length = map.iter().position(|&c| c == b'\n').unwrap() + 1;
    let start = ((start % (line_length)), (start / line_length));
    map[start.0 + start.1 * line_length] = b'a';
    let end = map.iter().position(|&c| c == b'E').unwrap();
    let end = ((end % line_length), (end / line_length));
    map[end.0 + end.1 * line_length] = b'z';
    (map, line_length, start, end)
}

pub fn part_one(input: &str) -> u32 {
    let (map, line_length, start, end) = parse(input);
    search(&map, line_length, start, end).unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let (map, line_length, _start, end) = parse(input);

    search_rev(&map, line_length, end, b'a').unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_one(&input), 31);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_two(&input), 29);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 12);
        assert_eq!(part_one(&input), 394);
    }

    #[test]
    fn solutions_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 12);
        assert_eq!(part_two(&input), 388);
    }
}
