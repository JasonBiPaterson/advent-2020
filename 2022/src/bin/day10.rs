use std::fs;

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

fn parse(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|l| {
            let tmp: nom::IResult<_, _> = nom::sequence::separated_pair(
                nom::bytes::complete::tag("addx"),
                nom::bytes::complete::tag(" "),
                nom::character::complete::i32,
            )(l);
            match tmp {
                Ok((_, (_, n))) => Instruction::Addx(n),
                Err(_) => Instruction::Noop,
            }
        })
        .collect()
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut cycles = vec![];
    let mut x = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Addx(n) => {
                cycles.push(x);
                cycles.push(x);
                x += n;
            }
            Instruction::Noop => {
                cycles.push(x);
            }
        }
    }

    let mut ans = 0;
    for n in [20i32, 60, 100, 140, 180, 220] {
        ans += cycles[(n - 1) as usize] * n;
    }
    ans
}

fn part2(instructions: &[Instruction]) -> Vec<String> {
    let mut cycles = vec![];
    let mut x = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Addx(n) => {
                cycles.push(x);
                cycles.push(x);
                x += n;
            }
            Instruction::Noop => {
                cycles.push(x);
            }
        }
    }

    let mut crt = vec![];
    for sub_cycle in cycles.chunks(40) {
        let mut crt_row = String::new();
        for (i, x) in sub_cycle.iter().enumerate() {
            let i_i32 = i as i32;
            if x - 1 <= i_i32 && i_i32 < x + 2 {
                crt_row.push('#');
            } else {
                crt_row.push('.');
            }
        }
        crt.push(crt_row);
    }
    crt
}

fn main() {
    let input: String = fs::read_to_string("input/day10").expect("Failed to read input");
    let motions = parse(&input);

    println!("Part 1: {}", part1(&motions));
    println!("Part 2:");
    for s in part2(&motions) {
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: String = fs::read_to_string("input/day10_test").expect("Failed to read input");
        let motions = parse(&input);

        assert_eq!(part1(&motions), 13140);
    }

    #[test]
    fn test_part2() {
        let input: String = fs::read_to_string("input/day10_test").expect("Failed to read input");
        let motions = parse(&input);
        let crt = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];

        assert_eq!(part2(&motions), crt);
    }
}
