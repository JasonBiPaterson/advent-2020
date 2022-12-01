use std::fs;

fn part1(input: &str) -> u32 {
    let lines = input.lines();

    let mut max_cals: u32 = 0;
    let mut current_cals: u32 = 0;

    for line in lines {
        if line.is_empty() {
            if current_cals > max_cals {
                max_cals = current_cals;
            }
            current_cals = 0;
        } else {
            let n: u32 = line.parse().expect("Not a number");
            current_cals += n;
        }
    }
    max_cals
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();

    let mut cals: Vec<u32> = vec![0];

    for line in lines {
        if line.is_empty() {
            cals.push(0);
        } else if let Some(n) = cals.last_mut() {
            *n += line.parse::<u32>().expect("Not a number");
        }
    }
    cals.sort_unstable();
    cals.iter().rev().take(3).sum()
}

fn main() {
    let input: String = fs::read_to_string("input/day01").expect("Failed to read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }
}
