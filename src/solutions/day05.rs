use crate::int_from_bytes;

pub fn part_one(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let size = (stacks.find('\n').unwrap() + 1) / 4;
    let mut vecs: Vec<Vec<u8>> = Vec::with_capacity(size);

    for _ in 0..size {
        vecs.push(vec![]);
    }

    stacks.lines().rev().skip(1).for_each(|line| {
        line.as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, &b)| b != b' ')
            .for_each(|(i, &b)| vecs[i].push(b))
    });

    moves
        .as_bytes()
        .split(|b| matches!(b, b' ' | b'\n'))
        .skip(1)
        .step_by(2)
        .map(|n| int_from_bytes::<usize>(n))
        .array_chunks::<3>()
        .for_each(|[count, from, to]| {
            let (f, t) = if from < to {
                let (x, y) = vecs.split_at_mut(to - 1);
                (&mut x[from - 1], &mut y[0])
            } else {
                let (x, y) = vecs.split_at_mut(from - 1);
                (&mut y[0], &mut x[to - 1])
            };
            let size = f.len() - count;
            f[size..].reverse();
            t.extend_from_slice(&f[size..]);
            f.truncate(size);
        });
    String::from_utf8(vecs.iter().map(|list| *list.last().unwrap()).collect()).unwrap()
}

pub fn part_two(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let size = (stacks.find('\n').unwrap() + 1) / 4;
    let mut vecs: Vec<Vec<u8>> = Vec::with_capacity(size);

    for _ in 0..size {
        vecs.push(vec![]);
    }

    stacks.lines().rev().skip(1).for_each(|line| {
        line.as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, &b)| b != b' ')
            .for_each(|(i, &b)| vecs[i].push(b))
    });

    moves
        .as_bytes()
        .split(|b| matches!(b, b' ' | b'\n'))
        .skip(1)
        .step_by(2)
        .map(|n| int_from_bytes::<usize>(n))
        .array_chunks::<3>()
        .for_each(|[count, from, to]| {
            let (f, t) = if from < to {
                let (x, y) = vecs.split_at_mut(to - 1);
                (&mut x[from - 1], &mut y[0])
            } else {
                let (x, y) = vecs.split_at_mut(from - 1);
                (&mut y[0], &mut x[to - 1])
            };
            let size = f.len() - count;
            t.extend_from_slice(&f[size..]);
            f.truncate(size);
        });
    String::from_utf8(vecs.iter().map(|list| *list.last().unwrap()).collect()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), "CMZ");
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), "MCD");
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 5);
        assert_eq!(part_one(&input), "LBLVVTVLP");
    }

    #[test]
    fn solution_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 5);
        assert_eq!(part_two(&input), "TPFFBDRJD");
    }
}
