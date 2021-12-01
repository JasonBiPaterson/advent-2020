use std::fs;

fn part1(entries: &[i32]) -> usize {
    entries.windows(2).filter(|ns| ns[1] > ns[0]).count()
}

fn part2(entries: &[i32]) -> usize {
    let sums: Vec<i32> = entries.windows(3).map(|w| w.iter().sum::<i32>()).collect();

    part1(&sums)
}

fn main() {
    let input: String = fs::read_to_string("input/day01").expect("Failed to read input");

    let entries: Vec<i32> = input
        .lines()
        .map(|line| line.parse().expect("Not a number"))
        .collect();

    println!("Part 1: {}", part1(&entries));
    println!("Part 2: {}", part2(&entries));
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
