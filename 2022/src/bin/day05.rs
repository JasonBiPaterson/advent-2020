use std::fs;

use nom::{bytes, character, sequence, IResult};

type CrateStack = Vec<char>;
type Procedure = (usize, usize, usize);

fn parse(input: &str) -> (Vec<CrateStack>, Vec<Procedure>) {
    let (crate_str, procedure_str) = input.split_once("\n\n").unwrap();

    let mut crate_lines = crate_str.lines().rev();
    let mut crate_stacks: Vec<CrateStack> = vec![];
    for (n, c) in crate_lines.next().unwrap().chars().enumerate() {
        if c.is_numeric() {
            let c_num: usize = c.to_string().parse().unwrap();
            crate_stacks.push(vec![]);
            for line in crate_lines.clone() {
                let maybe_crate = line.chars().nth(n).unwrap();
                if maybe_crate.is_uppercase() {
                    crate_stacks[c_num - 1].push(maybe_crate);
                }
            }
        }
    }

    let procedures: Vec<Procedure> = procedure_str
        .lines()
        .map(|line| {
            let tmp: IResult<_, _> = sequence::tuple((
                bytes::complete::tag("move "),
                character::complete::u32,
                bytes::complete::tag(" from "),
                character::complete::u32,
                bytes::complete::tag(" to "),
                character::complete::u32,
            ))(line);
            let (_, (_, n, _, a, _, b)) = tmp.unwrap();
            (n as usize, a as usize, b as usize)
        })
        .collect();

    (crate_stacks, procedures)
}

fn part1(mut crate_stacks: Vec<CrateStack>, procedures: &[Procedure]) -> String {
    for &(n, a, b) in procedures {
        for _ in 0..n {
            let cr = crate_stacks[a - 1].pop().unwrap();
            crate_stacks[b - 1].push(cr);
        }
    }
    crate_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn part2(mut crate_stacks: Vec<CrateStack>, procedures: &[Procedure]) -> String {
    for &(n, a, b) in procedures {
        let cr_a = crate_stacks.get_mut(a - 1).unwrap();
        let i = cr_a.len() - n;
        let mut moved = cr_a.drain(i..).collect();
        crate_stacks.get_mut(b - 1).unwrap().append(&mut moved);
    }
    crate_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn main() {
    let input: String = fs::read_to_string("input/day05").expect("Failed to read input");
    let (crate_stacks, procedures) = parse(&input);

    println!("Part 1: {}", part1(crate_stacks.clone(), &procedures));
    println!("Part 2: {}", part2(crate_stacks, &procedures));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let (crate_stacks, procedures) = parse(TEST_INPUT);
        assert_eq!(part1(crate_stacks, &procedures), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (crate_stacks, procedures) = parse(TEST_INPUT);
        assert_eq!(part2(crate_stacks, &procedures), "MCD");
    }
}
