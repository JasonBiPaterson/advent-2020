use std::{cmp, fs};

fn common_bit(words: &[&str], i: usize) -> cmp::Ordering {
    let mut count = 0;
    for &word in words {
        match &word.chars().nth(i) {
            Some('1') => count += 1,
            Some('0') => count -= 1,
            _ => panic!("Unexpected char"),
        };
    }
    count.cmp(&0)
}

fn part1(words: &[&str]) -> u32 {
    let len = words[0].chars().count();
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for i in 0..len {
        match common_bit(words, i) {
            cmp::Ordering::Less => {
                gamma.push('0');
                epsilon.push('1');
            }
            cmp::Ordering::Greater => {
                gamma.push('1');
                epsilon.push('0');
            }
            cmp::Ordering::Equal => panic!(),
        }
    }
    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn find_rating(words: &[&str], bit_match: (char, char, char)) -> u32 {
    let len = words[0].chars().count();
    let mut remaining_words = words.to_vec();
    for i in 0..len {
        let bit_char = match common_bit(&remaining_words, i) {
            cmp::Ordering::Less => bit_match.0,
            cmp::Ordering::Equal => bit_match.1,
            cmp::Ordering::Greater => bit_match.2,
        };
        remaining_words = remaining_words
            .iter()
            .filter(|&word| word.chars().nth(i) == Some(bit_char))
            .map(|&m| m)
            .collect::<Vec<&str>>();
        if remaining_words.len() == 1 {
            break;
        }
    }
    u32::from_str_radix(remaining_words[0], 2).unwrap()
}

fn part2(words: &[&str]) -> u32 {
    let oxygen = find_rating(&words, ('0', '1', '1'));
    let carbon_dioxide = find_rating(&words, ('1', '0', '0'));
    oxygen * carbon_dioxide
}

fn main() {
    let input: String = fs::read_to_string("input/day03").expect("Failed to read input");

    let words: Vec<&str> = input.trim().split_whitespace().collect();

    println!("Part 1: {}", part1(&words));
    println!("Part 2: {}", part2(&words));
}

#[cfg(test)]
mod tests {
    use super::*;
    const WORDS: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
        "11001", "00010", "01010",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&WORDS), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&WORDS), 230);
    }
}
