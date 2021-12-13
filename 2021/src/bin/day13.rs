use std::{collections::HashSet, fs};

type Dot = (usize, usize);
type Instruction = (char, usize);

fn parse_input(input: &str) -> (Vec<Dot>, Vec<Instruction>) {
    let (dot_input, instruction_input) = input.trim().split_once("\n\n").unwrap();
    let dots = dot_input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let instructions = instruction_input
        .lines()
        .map(|line| line.split_once('=').unwrap())
        .map(|(s, n)| (s.chars().rev().next().unwrap(), n.parse().unwrap()))
        .collect();
    (dots, instructions)
}

fn print_dots(dot_arr: &[Vec<bool>]) {
    for line in dot_arr {
        for &b in line {
            if b {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold(dots: &HashSet<Dot>, instruction: Instruction) -> HashSet<Dot> {
    let mut folded_paper = HashSet::new();
    let (direction, n) = instruction;
    for &dot in dots {
        match direction {
            'x' => {
                if dot.0 <= n {
                    folded_paper.insert(dot);
                } else {
                    folded_paper.insert(((2 * n - dot.0), dot.1));
                }
            }
            'y' => {
                if dot.1 <= n {
                    folded_paper.insert(dot);
                } else {
                    folded_paper.insert((dot.0, (2 * n - dot.1)));
                }
            }
            _ => panic!(),
        }
    }
    folded_paper
}

fn part1(dots: &[Dot], instructions: &[Instruction]) -> usize {
    let paper: HashSet<Dot> = dots.iter().copied().collect();
    fold(&paper, instructions[0]).len()
}

fn part2(dots: &[Dot], instructions: &[Instruction]) -> Vec<Vec<bool>> {
    let mut paper: HashSet<Dot> = dots.iter().copied().collect();
    for &instruction in instructions {
        paper = fold(&paper, instruction);
    }
    let x_max = instructions
        .iter()
        .rev()
        .find(|&&(direction, _)| direction == 'x')
        .unwrap()
        .1;
    let y_max = instructions
        .iter()
        .rev()
        .find(|&&(direction, _)| direction == 'y')
        .unwrap()
        .1;
    let mut dotarr = vec![vec![false; x_max]; y_max];
    for dot in paper {
        dotarr[dot.1][dot.0] = true;
    }
    dotarr
}

fn main() {
    let input: String = fs::read_to_string("input/day13").expect("Failed to read input");

    let (dots, instructions) = parse_input(&input);

    println!("Part 1: {}", part1(&dots, &instructions));
    println!("Part 2:");
    print_dots(&part2(&dots, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_part1() {
        let (dots, instructions) = parse_input(INPUT);
        assert_eq!(part1(&dots, &instructions), 17);
    }

    #[test]
    fn test_part2() {
        let (dots, instructions) = parse_input(INPUT);
        let ans = part2(&dots, &instructions);
        print_dots(&ans);
        assert_eq!(
            ans,
            [
                [true, true, true, true, true],
                [true, false, false, false, true],
                [true, false, false, false, true],
                [true, false, false, false, true],
                [true, true, true, true, true],
                [false, false, false, false, false],
                [false, false, false, false, false]
            ]
        );
    }
}
