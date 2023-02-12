use nom::{bytes::complete::tag, IResult, Parser};
use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.cmp(&vec![Packet::Int(*b)]),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].cmp(b),
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    nom::branch::alt((
        nom::sequence::delimited(
            tag("["),
            nom::multi::separated_list0(tag(","), parse_packet),
            tag("]"),
        )
        .map(Packet::List),
        nom::character::complete::u32.map(Packet::Int),
    ))(input)
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|s| s.split_once("\n").unwrap())
        .map(|(a, b)| (parse_packet(a).unwrap().1, parse_packet(b).unwrap().1))
        .collect()
}

fn part1(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|&(_, (a, b))| a <= b)
        .map(|(n, _)| n + 1)
        .sum()
}

fn part2(pairs: &[(Packet, Packet)]) -> usize {
    let divider_packet1 = parse_packet("[[2]]").unwrap().1;
    let divider_packet2 = parse_packet("[[6]]").unwrap().1;
    let mut coll = vec![&divider_packet1, &divider_packet2];
    for (a, b) in pairs {
        coll.push(a);
        coll.push(b);
    }
    coll.sort();
    coll.iter()
        .enumerate()
        .filter(|&(_, p)| *p == &divider_packet1 || *p == &divider_packet2)
        .map(|(n, _)| n + 1)
        .product()
}

fn main() {
    let input: String = fs::read_to_string("input/day13").expect("Failed to read input");
    let pairs = parse(&input);

    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&TEST_INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&TEST_INPUT)), 140);
    }
}
