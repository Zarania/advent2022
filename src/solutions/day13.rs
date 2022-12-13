use std::cmp::Ordering;

use crate::int_from_bytes_exact;
#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Interger(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Interger(a), Interger(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Interger(_), List(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Interger(a), List(_)) => List(vec![Interger(*a)]).cmp(other),
            (_, Interger(_)) => other.cmp(self).reverse(),
        }
    }
}

impl Packet {
    fn get_data(data: &[u8]) -> Packet {
        if data[0] == b'[' {
            let mut stack: i32 = 0;
            Packet::List(
                data[1..data.len() - 1]
                    .split(|&c| {
                        if c == b'[' {
                            stack += 1
                        } else if c == b']' {
                            stack -= 1
                        }
                        c == b',' && stack == 0
                    })
                    .filter_map(|s| (!s.is_empty()).then(|| Self::get_data(s)))
                    .collect(),
            )
        } else {
            Packet::Interger(int_from_bytes_exact(data))
        }
    }

    fn new(data: &[u8]) -> Self {
        Self::get_data(data)
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(Packet::new)
        .array_chunks::<2>()
        .enumerate()
        .filter(|(_, [packet1, packet2])| packet1 < packet2)
        .map(|(index, _)| index + 1)
        .sum::<usize>() as u32
}

pub fn part_two(input: &str) -> u32 {
    let mut packets = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(Packet::new)
        .collect::<Vec<_>>();

    let two = Packet::List(vec![Packet::Interger(2)]);
    let six = Packet::List(vec![Packet::Interger(6)]);
    packets.push(two.clone());
    packets.push(six.clone());

    packets.sort_unstable();
    ((packets.binary_search(&two).unwrap() + 1) * (packets.binary_search(&six).unwrap() + 1)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_two(&input), 140);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 13);
        assert_eq!(part_one(&input), 5292);
    }

    #[test]
    fn solutions_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 13);
        assert_eq!(part_two(&input), 23868);
    }
}
