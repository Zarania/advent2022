use crate::int_from_bytes_exact;

enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    true_index: usize,
    false_index: usize,
    inspect_count: usize,
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, f: impl Fn(usize) -> usize) -> usize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let temp_monkey = &monkeys[i];
            let false_index = temp_monkey.false_index;
            let true_index = temp_monkey.true_index;
            if let Ok([monkey, false_monkey, true_monkey]) =
                monkeys.get_many_mut([i, false_index, true_index])
            {
                monkey.inspect_count += monkey.items.len();
                for mut item in monkey.items.drain(..) {
                    item = match monkey.operation {
                        Operation::Add(x) => f(item + x),
                        Operation::Multiply(x) => f(item * x),
                        Operation::Square => f(item * item),
                    };

                    if item % monkey.test == 0 {
                        true_monkey.items.push(item);
                    } else {
                        false_monkey.items.push(item);
                    }
                }
            }
        }
    }
    monkeys.sort_unstable_by(|m1, m2| m2.inspect_count.cmp(&m1.inspect_count));
    monkeys.iter().take(2).map(|m| m.inspect_count).product()
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .chain(["\n"; 1])
        .array_chunks::<7>()
        .map(|monkey| {
            let bytes = monkey[2].as_bytes();
            let sign = bytes[23];
            let operand;
            let operation = if bytes[25] == b'o' {
                Operation::Square
            } else {
                operand = int_from_bytes_exact::<usize>(&bytes[25..]);
                match sign {
                    b'*' => Operation::Multiply(operand),
                    b'+' => Operation::Add(operand),
                    _ => unreachable!(),
                }
            };
            Monkey {
                items: monkey[1]
                    .as_bytes()
                    .group_by(|_, b| *b != b',')
                    .map(int_from_bytes_exact::<usize>)
                    .collect(),
                operation,
                test: int_from_bytes_exact::<usize>(&monkey[3].as_bytes()[21..]),
                true_index: int_from_bytes_exact::<usize>(&monkey[4].as_bytes()[29..]),
                false_index: int_from_bytes_exact::<usize>(&monkey[5].as_bytes()[29..]),
                inspect_count: 0,
            }
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> usize {
    solve(parse(input), 20, |x| x / 3)
}

pub fn part_two(input: &str) -> usize {
    let monkeys = parse(input);

    let total_mod: usize = monkeys.iter().map(|m| m.test).product();

    solve(parse(input), 10000, |x| x % total_mod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_one(&input), 10605);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_two(&input), 2713310158);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 11);
        assert_eq!(part_one(&input), 76728);
    }

    #[test]
    fn solutions_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 11);
        assert_eq!(part_two(&input), 21553910156);
    }
}
