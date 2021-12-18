use std::{fs, ops::Add};

use nom::{branch, bytes::complete::tag, character, combinator, sequence};

#[derive(Debug, Clone)]
enum SnailFish {
    Number(u32),
    Pair(Box<SnailFish>, Box<SnailFish>),
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SnailFish::Pair(Box::new(self), Box::new(rhs))
    }
}

impl SnailFish {
    fn add_leftmost(&mut self, n: u32) {
        match self {
            SnailFish::Number(num) => *num += n,
            SnailFish::Pair(a, _) => a.add_leftmost(n),
        }
    }

    fn add_rightmost(&mut self, n: u32) {
        match self {
            SnailFish::Number(num) => *num += n,
            SnailFish::Pair(_, b) => b.add_rightmost(n),
        }
    }

    fn add_left(&mut self, n: u32) {
        match self {
            SnailFish::Number(num) => *num += n,
            SnailFish::Pair(a, _) => a.add_rightmost(n),
        }
    }

    fn add_right(&mut self, n: u32) {
        match self {
            SnailFish::Number(num) => *num += n,
            SnailFish::Pair(_, b) => b.add_leftmost(n),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            SnailFish::Number(_) => None,
            SnailFish::Pair(a, b) => {
                if let SnailFish::Number(n) = **a {
                    if let SnailFish::Number(m) = **b {
                        if depth == 4 {
                            *self = SnailFish::Number(0);
                            return Some((Some(n), Some(m)));
                        }
                        return None;
                    }
                }
                if let Some(t) = a.explode(depth + 1) {
                    if let (a, Some(n)) = t {
                        self.add_right(n);
                        return Some((a, None));
                    }
                    return Some(t);
                }
                if let Some(t) = b.explode(depth + 1) {
                    if let (Some(n), b) = t {
                        self.add_left(n);
                        return Some((None, b));
                    }
                    return Some(t);
                }
                None
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailFish::Number(n) => {
                if *n >= 10 {
                    *self = SnailFish::Pair(
                        Box::new(SnailFish::Number(*n / 2)),
                        Box::new(SnailFish::Number((*n + 1) / 2)),
                    );
                    return true;
                }
                false
            }
            SnailFish::Pair(a, b) => {
                if a.split() {
                    return true;
                }
                if b.split() {
                    return true;
                }
                false
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailFish::Number(n) => *n,
            SnailFish::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn reduce(self) -> Self {
        let mut sf = self;
        loop {
            if sf.explode(0).is_some() {
                continue;
            }
            if sf.split() {
                continue;
            }
            break;
        }
        sf
    }
}

fn parse_snailfish(input: &str) -> nom::IResult<&str, SnailFish> {
    branch::alt((
        combinator::map(character::complete::u32, SnailFish::Number),
        combinator::map(
            sequence::delimited(
                tag("["),
                sequence::separated_pair(parse_snailfish, tag(","), parse_snailfish),
                tag("]"),
            ),
            |(a, b)| SnailFish::Pair(Box::new(a), Box::new(b)),
        ),
    ))(input)
}

fn parse_input(input: &str) -> Vec<SnailFish> {
    input
        .trim()
        .lines()
        .map(|line| parse_snailfish(line.trim()).unwrap().1)
        .collect()
}

fn part1(fishes: &[SnailFish]) -> u32 {
    fishes
        .to_owned()
        .into_iter()
        .reduce(|a, b| (a + b).reduce())
        .unwrap()
        .magnitude()
}

fn part2(fishes: &[SnailFish]) -> u32 {
    fishes
        .to_vec()
        .into_iter()
        .enumerate()
        .flat_map(|(i, a)| {
            fishes
                .to_vec()
                .into_iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .map(|(_, b)| (a.clone() + b).reduce().magnitude())
                .collect::<Vec<u32>>()
        })
        .max()
        .unwrap()
}

fn main() {
    let input: String = fs::read_to_string("input/day18").expect("Failed to read input");

    let parsed_input = parse_input(&input);

    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";

    #[test]
    fn test_part1() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(part1(&parsed_input), 4140);
    }

    #[test]
    fn test_part2() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(part2(&parsed_input), 3993);
    }
}
