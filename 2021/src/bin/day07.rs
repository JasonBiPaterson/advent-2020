use std::{cmp, fs, ops};

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn crab_range(crabs: &[u32]) -> ops::Range<u32> {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    min..max
}

fn abs_diff(n: u32, m: u32) -> u32 {
    match n.cmp(&m) {
        cmp::Ordering::Less => m - n,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => n - m,
    }
}

fn part1(crabs: &[u32]) -> u32 {
    crab_range(crabs)
        .map(|n| crabs.iter().map(|&crab| abs_diff(crab, n)).sum())
        .min()
        .unwrap()
}

fn part2(crabs: &[u32]) -> u32 {
    crab_range(crabs)
        .map(|n| {
            crabs
                .iter()
                .map(|&crab| abs_diff(crab, n))
                .map(|n| n * (n + 1) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let input: String = fs::read_to_string("input/day07").expect("Failed to read input");

    let crabs = parse_input(&input);

    println!("Part 1: {}", part1(&crabs));
    println!("Part 2: {}", part2(&crabs));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let crabs = parse_input(INPUT);
        assert_eq!(part1(&crabs), 37);
    }

    #[test]
    fn test_part2() {
        let crabs = parse_input(INPUT);
        assert_eq!(part2(&crabs), 168);
    }
}
