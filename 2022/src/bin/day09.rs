use std::{collections::HashSet, fs};

fn parse(s: &str) -> Vec<(char, u32)> {
    s.lines()
        .map(|l| {
            let tmp: nom::IResult<_, _, nom::error::Error<_>> = nom::sequence::separated_pair(
                nom::character::complete::anychar,
                nom::bytes::complete::tag(" "),
                nom::character::complete::u32,
            )(l);
            tmp.unwrap().1
        })
        .collect()
}

fn part1(motions: &[(char, u32)]) -> usize {
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);

    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();

    for &(d, n) in motions {
        let step = match d {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..n {
            head_pos = (head_pos.0 + step.0, head_pos.1 + step.1);

            let x_diff = head_pos.0 - tail_pos.0;
            let y_diff = head_pos.1 - tail_pos.1;
            if !((-1..=1).contains(&x_diff) && (-1..=1).contains(&y_diff)) {
                tail_pos = (head_pos.0 - (x_diff / 2), head_pos.1 - (y_diff / 2));
            }

            visited_pos.insert(tail_pos);
        }
    }
    visited_pos.len()
}

fn part2(motions: &[(char, u32)]) -> usize {
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();

    for &(d, n) in motions {
        let step = match d {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..n {
            rope[0] = (rope[0].0 + step.0, rope[0].1 + step.1);

            for i in 1..(rope.len()) {
                let head_pos = rope[i - 1];
                let tail_pos = rope[i];

                let x_diff = head_pos.0 - tail_pos.0;
                let y_diff = head_pos.1 - tail_pos.1;
                if !((-1..=1).contains(&x_diff) && (-1..=1).contains(&y_diff)) {
                    rope[i] = (head_pos.0 - (x_diff / 2), head_pos.1 - (y_diff / 2));
                }
            }

            visited_pos.insert(*rope.last().unwrap());
        }
    }
    visited_pos.len()
}

fn main() {
    let input: String = fs::read_to_string("input/day09").expect("Failed to read input");
    let motions = parse(&input);

    println!("Part 1: {}", part1(&motions));
    println!("Part 2: {}", part2(&motions));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT1)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT1)), 1);
        assert_eq!(part2(&parse(TEST_INPUT2)), 36);
    }
}
