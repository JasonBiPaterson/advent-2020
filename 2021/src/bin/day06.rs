use std::fs;

fn stepn(fish_timers: &[u64; 9], n: u64) -> u64 {
    let mut timers = *fish_timers;
    for _ in 0..n {
        let tmp_timers = timers;
        timers = [0u64; 9];
        for (i, n) in tmp_timers.iter().enumerate() {
            match i {
                0 => {
                    timers[6] += *n;
                    timers[8] += *n
                }
                _ => timers[i - 1] += *n,
            }
        }
    }
    timers.iter().sum()
}

fn parse_input(input: &str) -> [u64; 9] {
    let mut fish_timers = [0u64; 9];
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|i| fish_timers[i] += 1);
    fish_timers
}

fn main() {
    let input: String = fs::read_to_string("input/day06").expect("Failed to read input");

    let fish_timers = parse_input(&input);

    println!("Part 1: {}", stepn(&fish_timers, 80));
    println!("Part 2: {}", stepn(&fish_timers, 256));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let timers = parse_input(INPUT);
        assert_eq!(stepn(&timers, 18), 26);
        assert_eq!(stepn(&timers, 80), 5934);
    }

    #[test]
    fn test_part2() {
        let timers = parse_input(INPUT);
        assert_eq!(stepn(&timers, 256), 26984457539);
    }
}
