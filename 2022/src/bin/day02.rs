use std::fs;

fn parse(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|chars| (chars[0], chars[2]))
        .collect()
}

fn part1(guide: &[(char, char)]) -> u32 {
    let mut score: u32 = 0;
    for instruction in guide {
        match instruction {
            (_, 'X') => score += 1, // Rock
            (_, 'Y') => score += 2, // Paper
            (_, 'Z') => score += 3, // Scissors
            _ => (),
        }
        match instruction {
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => score += 0, // Lose
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => score += 3, // Draw
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => score += 6, // Win
            _ => (),
        }
    }
    score
}

fn part2(guide: &[(char, char)]) -> u32 {
    let mut score: u32 = 0;
    for instruction in guide {
        match instruction {
            (_, 'X') => score += 0, // Lose
            (_, 'Y') => score += 3, // Draw
            (_, 'Z') => score += 6, // Win
            _ => (),
        }
        match instruction {
            ('A', 'Y') | ('B', 'X') | ('C', 'Z') => score += 1, // Rock
            ('A', 'Z') | ('B', 'Y') | ('C', 'X') => score += 2, // Paper
            ('A', 'X') | ('B', 'Z') | ('C', 'Y') => score += 3, // Scissors
            _ => (),
        }
    }
    score
}

fn main() {
    let input: String = fs::read_to_string("input/day02").expect("Failed to read input");
    let guide = parse(&input);

    println!("Part 1: {:?}", part1(&guide));
    println!("Part 2: {:?}", part2(&guide));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 12);
    }
}
