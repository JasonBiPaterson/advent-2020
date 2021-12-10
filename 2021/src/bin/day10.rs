use std::fs;

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn part1(lines: &[&str]) -> u32 {
    let mut errs: [u32; 4] = [0, 0, 0, 0];
    for &line in lines {
        let mut ch_stack: Vec<char> = vec![];
        for ch in line.chars() {
            if matches!(ch, '(' | '[' | '{' | '<') {
                ch_stack.push(ch);
                continue;
            }
            let ch_pop = ch_stack.pop();
            match ch {
                ')' if ch_pop == Some('(') => continue,
                ']' if ch_pop == Some('[') => continue,
                '}' if ch_pop == Some('{') => continue,
                '>' if ch_pop == Some('<') => continue,
                ')' => {
                    errs[0] += 1;
                    break;
                }
                ']' => {
                    errs[1] += 1;
                    break;
                }
                '}' => {
                    errs[2] += 1;
                    break;
                }
                '>' => {
                    errs[3] += 1;
                    break;
                }
                _ => panic!(),
            }
        }
    }
    errs[0] * 3 + errs[1] * 57 + errs[2] * 1197 + errs[3] * 25137
}

fn part2(lines: &[&str]) -> u64 {
    let mut scores: Vec<u64> = vec![];
    'lines: for &line in lines {
        let mut ch_stack: Vec<char> = vec![];
        for ch in line.chars() {
            if matches!(ch, '(' | '[' | '{' | '<') {
                ch_stack.push(ch);
                continue;
            }
            let ch_pop = ch_stack.pop();
            match ch {
                ')' if ch_pop == Some('(') => continue,
                ']' if ch_pop == Some('[') => continue,
                '}' if ch_pop == Some('{') => continue,
                '>' if ch_pop == Some('<') => continue,
                ')' | ']' | '}' | '>' => continue 'lines,
                _ => panic!(),
            }
        }
        scores.push(ch_stack.iter().rev().fold(0_u64, |score, ch| match ch {
            '(' => score * 5 + 1,
            '[' => score * 5 + 2,
            '{' => score * 5 + 3,
            '<' => score * 5 + 4,
            _ => score,
        }));
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input: String = fs::read_to_string("input/day10").expect("Failed to read input");

    let lines = parse_input(&input);

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_part1() {
        let lines = parse_input(INPUT);
        assert_eq!(part1(&lines), 26397);
    }

    #[test]
    fn test_part2() {
        let lines = parse_input(INPUT);
        assert_eq!(part2(&lines), 288_957);
    }
}
