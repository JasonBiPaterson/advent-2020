use std::fs;

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn parts(lines: &[&str]) -> (u32, u64) {
    let mut scores: Vec<u64> = vec![];
    let mut part1: u32 = 0;
    'lines: for &line in lines {
        let mut ch_stack: Vec<char> = vec![];
        for ch in line.chars() {
            if matches!(ch, '(' | '[' | '{' | '<') {
                ch_stack.push(ch);
                continue;
            }
            let ch_pop = ch_stack.pop();
            match ch {
                ')' if ch_pop != Some('(') => part1 += 3,
                ']' if ch_pop != Some('[') => part1 += 57,
                '}' if ch_pop != Some('{') => part1 += 1197,
                '>' if ch_pop != Some('<') => part1 += 25137,
                ')' | ']' | '}' | '>' => continue,
                _ => panic!(),
            }
            continue 'lines;
        }
        scores.push(ch_stack.iter().rev().fold(0_u64, |score, ch| {
            score * 5
                + match ch {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                }
        }));
    }
    scores.sort_unstable();
    let part2 = scores[scores.len() / 2];
    (part1, part2)
}

fn main() {
    let input: String = fs::read_to_string("input/day10").expect("Failed to read input");

    let lines = parse_input(&input);
    let (part1, part2) = parts(&lines);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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
    fn test_parts() {
        let lines = parse_input(INPUT);
        let (part1, part2) = parts(&lines);
        assert_eq!(part1, 26397);
        assert_eq!(part2, 288_957);
    }
}
