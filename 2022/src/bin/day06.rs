use std::{fs, collections::HashSet};

fn find_marker(input: &str, len: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let mut n = len;
    for window in chars.windows(len) {
        let mut char_set: HashSet<char> = HashSet::new();
        if window.iter().all(|c| char_set.insert(*c)) {
            break;
        }
        n += 1;
    }
    n
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let input: String = fs::read_to_string("input/day06").expect("Failed to read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 19);
    }
}
