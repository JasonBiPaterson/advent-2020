use nom::{bytes, character, error, sequence, IResult};
use std::fs;

fn part1(input: &[Assignment]) -> usize {
    input
        .iter()
        .filter(|((a, b), (c, d))| (a <= c && d <= b) || (c <= a && b <= d))
        .count()
}

fn part2(input: &[Assignment]) -> usize {
    input
        .iter()
        .filter(|((a, b), (c, d))| (a <= c && c <= b) || (a <= d && d <= b) || (c <= a && b <= d))
        .count()
}

type Assignment = ((u32, u32), (u32, u32));

fn parse(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(|line| {
            let a: IResult<_, _, error::Error<_>> = sequence::separated_pair(
                sequence::separated_pair(
                    character::complete::u32,
                    bytes::complete::tag("-"),
                    character::complete::u32,
                ),
                bytes::complete::tag(","),
                sequence::separated_pair(
                    character::complete::u32,
                    bytes::complete::tag("-"),
                    character::complete::u32,
                ),
            )(line);
            a.unwrap().1
        })
        .collect()
}

fn main() {
    let input: String = fs::read_to_string("input/day04").expect("Failed to read input");
    let assignments = parse(&input);
    println!("Part 1: {}", part1(&assignments));
    println!("Part 2: {}", part2(&assignments));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 4);
    }
}
