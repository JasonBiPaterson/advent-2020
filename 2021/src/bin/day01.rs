use std::fs;

fn part1(entries: &[i32]) -> usize {
    entries
        .iter()
        .zip(entries.iter().skip(1))
        .filter(|(&a, &b)| -> bool { b > a })
        .collect::<Vec<(&i32, &i32)>>()
        .len()
}

fn part2(entries: &[i32]) -> usize {
    let sums: Vec<i32> = entries
        .iter()
        .zip(entries.iter().skip(1))
        .map(|(a, b)| -> i32 { a + b })
        .zip(entries.iter().skip(2))
        .map(|(a, b)| -> i32 { a + b })
        .collect();

    part1(&sums)
}

fn main() {
    let input: String = fs::read_to_string("input/day01").expect("Failed to read input");

    let entries: Vec<i32> = input
        .lines()
        .map(|line| line.parse().expect("Not a number"))
        .collect();

    println!("{}", part1(&entries));
    println!("{}", part2(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 5);
    }
}
