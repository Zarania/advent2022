use nom::{
    branch::alt, character::complete as ch, combinator::map, multi::separated_list0,
    sequence::delimited, Finish, IResult,
};
use std::cmp::Ordering;

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

fn parse(input: &str) -> Vec<Packet> {
    fn parse(line: &str) -> IResult<&str, Packet> {
        alt((
            delimited(
                ch::char('['),
                map(separated_list0(ch::char(','), parse), Packet::List),
                ch::char(']'),
            ),
            map(ch::u32, Packet::Interger),
        ))(line)
    }
    input
        .lines()
        .filter_map(|l| parse(l).finish().map(|x| x.1).ok())
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    parse(input)
        .array_chunks::<2>()
        .enumerate()
        .filter(|(_, [packet1, packet2])| packet1 < packet2)
        .map(|(index, _)| index + 1)
        .sum::<usize>() as u32
}

pub fn part_two(input: &str) -> u32 {
    let mut packets = parse(input);

    let [two, six] = &*parse("[[2]]\n[[6]]") else { unreachable!() };
    packets.push(two.clone());
    packets.push(six.clone());

    packets.sort_unstable();
    ((packets.binary_search(two).unwrap() + 1) * (packets.binary_search(six).unwrap() + 1)) as u32
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
