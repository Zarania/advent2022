use crate::int_from_bytes_greedy;

struct Directory {
    parent: usize,
    size: u32,
}

fn parse_data(input: &str) -> Vec<Directory> {
    let mut directories = vec![Directory { parent: 0, size: 0 }];
    let mut current = 0;

    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        //cd
        if b[0] == b'$' && b[2] == b'c' {
            //cd ..
            current = if b[5] == b'.' {
                directories[current].parent
            } else {
                //when we cd into a new directory, push it to our list
                directories.push(Directory {
                    parent: current,
                    size: 0,
                });
                directories.len() - 1
            }
        } else if b[0] != b'$' && b[0] != b'd' {
            //check first byte for $ to filter out $ ls, if it's not a directory it's a file
            let size = int_from_bytes_greedy::<u32>(b);
            directories[current].size += size;
        }
    });

    for i in (1..directories.len()).rev() {
        let parent = directories[i].parent;
        directories[parent].size += directories[i].size;
    }

    directories
}

pub fn part_one(input: &str) -> u32 {
    let tree = parse_data(input);

    tree.iter()
        .map(|d| d.size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let tree = parse_data(input);

    let available = 70_000_000;
    let need = 30_000_000;
    let used = tree[0].size;
    let free = available - used;

    let min_size_to_remove = need - free;
    tree.iter()
        .map(|d| d.size)
        .filter(|&size| min_size_to_remove <= size)
        .min()
        .unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 95437);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 24933642);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 7);
        assert_eq!(part_one(&input), 1367870);
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 7);
        assert_eq!(part_two(&input), 549173);
    }
}
