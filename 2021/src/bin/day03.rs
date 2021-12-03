use std::fs;

fn part1(words: &[&str]) -> u32 {
    let len = words[0].chars().count();
    let mut bit_count: Vec<i32> = vec![0; len];
    for word in words {
        for (i, c) in word.chars().enumerate() {
            match c {
                '1' => bit_count[i] += 1,
                '0' => bit_count[i] -= 1,
                _ => panic!("Unexpected char"),
            };
        }
    }
    let mut gamma: Vec<char> = vec!['0'; len];
    let mut epsilon: Vec<char> = vec!['1'; len];
    bit_count.iter().enumerate().for_each(|(i, n)| {
        if *n > 0 {
            gamma[i] = '1';
            epsilon[i] = '0';
        }
    });
    let gamma = u32::from_str_radix(&gamma.iter().collect::<String>(), 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon.iter().collect::<String>(), 2).unwrap();
    gamma * epsilon
}

fn part2(words: &[&str]) -> u32 {
    let len = words[0].chars().count();
    let mut remaining_words = words.to_vec();
    for i in 0..len {
        let mut count = 0;
        remaining_words.iter().for_each(|&word| {
            match &word.chars().nth(i) {
                Some('1') => count += 1,
                Some('0') => count -= 1,
                _ => panic!("Unexpected char"),
            };
        });
        remaining_words = remaining_words
            .iter()
            .filter(|&word| match word.chars().nth(i) {
                Some('1') => count >= 0,
                Some('0') => count < 0,
                _ => panic!(),
            })
            .map(|m| *m)
            .collect();
    }
    let oxygen = u32::from_str_radix(remaining_words[0], 2).unwrap();

    let mut remaining_words = words.to_vec();
    for i in 0..len {
        let mut count = 0;
        remaining_words.iter().for_each(|&word| {
            match &word.chars().nth(i) {
                Some('1') => count += 1,
                Some('0') => count -= 1,
                _ => panic!("Unexpected char"),
            };
        });
        remaining_words = remaining_words
            .iter()
            .filter(|&word| match word.chars().nth(i) {
                Some('1') => count < 0,
                Some('0') => count >= 0,
                _ => panic!(),
            })
            .map(|m| *m)
            .collect();
        if remaining_words.len() == 1 {
            break;
        }
    }
    let carbon_dioxide = u32::from_str_radix(remaining_words[0], 2).unwrap();
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

    #[test]
    fn test_part1() {
        let words = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(part1(&words), 198);
    }

    #[test]
    fn test_part2() {
        let words = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(part2(&words), 230);
    }
}
