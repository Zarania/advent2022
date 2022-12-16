use crate::int_from_bytes_greedy_signed;

struct Sensor {
    x: isize,
    y: isize,
    range: isize,
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut index = 12;
            let (sensor_x, skip) = int_from_bytes_greedy_signed::<isize>(&line[index..]);
            index += skip + 4;
            let (sensor_y, skip) = int_from_bytes_greedy_signed::<isize>(&line[index..]);
            index += skip + 25;
            let (beacon_x, skip) = int_from_bytes_greedy_signed::<isize>(&line[index..]);
            index += skip + 4;
            let (beacon_y, _) = int_from_bytes_greedy_signed::<isize>(&line[index..]);
            let distance = distance(sensor_x, beacon_x, sensor_y, beacon_y);
            Sensor {
                x: sensor_x,
                y: sensor_y,
                range: distance,
            }
        })
        .collect()
}

fn distance(x1: isize, x2: isize, y1: isize, y2: isize) -> isize {
    (y1.abs_diff(y2) + x1.abs_diff(x2)) as isize
}

pub fn part_one(input: &str) -> u32 {
    let sensors = parse(input);
    let line = if input.len() < 1000 { 10 } else { 2000000 };

    //collect the range on the line that the sensor doesn't match
    let mut empty_spaces = sensors
        .iter()
        .map(|s| (s.x, s.range, (s.y - line).abs()))
        .filter(|(_, range, dist)| dist <= range)
        .map(|(x, range, dist)| {
            let half = range - dist;
            (x - half)..=(x + half)
        })
        .collect::<Vec<_>>();

    //merge
    empty_spaces.sort_unstable_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut index = 0;
    for i in 1..(empty_spaces.len()) {
        if empty_spaces[index].contains(empty_spaces[i].start())
            || empty_spaces[i].contains(empty_spaces[index].start())
        {
            empty_spaces[index] = *empty_spaces[i].start().min(empty_spaces[index].start())
                ..=*empty_spaces[i].end().max(empty_spaces[index].end());
        } else {
            index += 1;
        }
    }
    empty_spaces.truncate(index + 1);

    let mut count = 0;
    for r in empty_spaces {
        count += r.end() - r.start();
    }

    count as u32
}

pub fn part_two(input: &str) -> usize {
    let sensors = parse(input);
    let size = if input.len() < 1000 { 20 } else { 4000000 };
    //do some transformation math here
    //https://math.stackexchange.com/questions/732679/how-to-rotate-a-matrix-by-45-degrees
    //rotating from diagonal to straight gives new_x = y - x, new_y = y + x
    //rotating back is y = (new_x + new_y) / 2, x = new_y - y
    //we are only checking the space directly outside the 4 corners of each rect that overlap with another diamond
    //this appears to be enough to solve the problem
    //possible things you would need to do for a more general solution include checking the 4 corners of the grid
    //and instead of only checking corners, check all points along the line
    let mut ascending_corners = sensors
        .iter()
        .flat_map(|s| [s.x - s.y - s.range - 1, s.x - s.y + s.range + 1])
        .collect::<Vec<_>>();

    let mut descending_corners = sensors
        .iter()
        .flat_map(|s| [s.x + s.y - s.range - 1, s.x + s.y + s.range + 1])
        .collect::<Vec<_>>();

    //by sorting the corner x values and keeping only the duplicates, we know that two things had
    //overlapping x values.  We can combine these later to vastly reduce the data set size
    ascending_corners.sort_unstable();
    descending_corners.sort_unstable();

    ascending_corners = ascending_corners
        .windows(2)
        .filter_map(|w| match w {
            [a, b] if a == b => Some(*a),
            _ => None,
        })
        .collect();

    descending_corners = descending_corners
        .windows(2)
        .filter_map(|w| match w {
            [a, b] if a == b => Some(*a),
            _ => None,
        })
        .collect();

    ascending_corners.dedup();
    descending_corners.dedup();

    //now that we have our list of overlapping x, loop through both
    for a in ascending_corners {
        for d in &descending_corners {
            //translate back from our rotated grid
            let y = (d - a) / 2;
            let intersection = (a + y, y);
            if intersection.0 < 0
                || intersection.0 > size
                || intersection.1 < 0
                || intersection.1 > size
            {
                continue;
            }

            if !sensors
                .iter()
                .any(|s| distance(s.x, intersection.0, s.y, intersection.1) <= s.range)
            {
                return intersection.0 as usize * 4000000 + intersection.1 as usize;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_one(&input), 26);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_two(&input), 56000011);
    }

    #[test]
    fn solution_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 15);
        assert_eq!(part_one(&input), 5461729);
    }

    #[test]
    fn solutions_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 15);
        assert_eq!(part_two(&input), 10621647166538);
    }
}
