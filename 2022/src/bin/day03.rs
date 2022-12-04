use std::{collections::HashMap, fs, str::Chars};

fn part1(input: &str) -> u32 {
    let mut dict: HashMap<char, u32> = HashMap::new();
    let mut n = 1;
    for c in 'a'..='z' {
        dict.insert(c, n);
        n += 1;
    }
    for c in 'A'..='Z' {
        dict.insert(c, n);
        n += 1;
    }

    let common_chars: Vec<char> = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| a.chars().find(|ca| b.chars().any(|cb| ca == &cb)).unwrap())
        .collect();

    let mut ans = 0;
    for c in common_chars {
        if let Some(n) = dict.get(&c) {
            ans += n;
        }
    }
    ans
}

fn part2(input: &str) -> u32 {
    let mut dict: HashMap<char, u32> = HashMap::new();
    let mut n = 1;
    for c in 'a'..='z' {
        dict.insert(c, n);
        n += 1;
    }
    for c in 'A'..='Z' {
        dict.insert(c, n);
        n += 1;
    }

    let mut ans = 0;
    let char_lines = input.lines().map(str::chars).collect::<Vec<Chars>>();
    for chunk in char_lines.chunks(3) {
        if let [chars1, chars2, chars3] = chunk {
            let c = chars1
                .clone()
                .filter(|&ca| chars2.clone().any(|cb| ca == cb))
                .find(|&ca| chars3.clone().any(|cb| ca == cb))
                .unwrap();
            if let Some(n) = dict.get(&c) {
                ans += n;
            }
        }
    }
    ans
}

fn main() {
    let input: String = fs::read_to_string("input/day03").expect("Failed to read input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}
