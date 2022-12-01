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
