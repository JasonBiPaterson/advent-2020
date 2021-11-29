use std::fs;

fn part1(entries: &[u32]) -> u32 {
    entries
        .iter()
        .find_map(|&a| {
            entries.iter().find_map(|&b| match a + b {
                2020 => Some(a * b),
                _ => None,
            })
        })
        .expect("None found")
}

fn part2(entries: &[u32]) -> u32 {
    entries
        .iter()
        .find_map(|&a| {
            entries.iter().find_map(|&b| {
                entries.iter().find_map(|&c| match a + b + c {
                    2020 => Some(a * b * c),
                    _ => None,
                })
            })
        })
        .expect("None found")
}

fn main() {
    let input: String = fs::read_to_string("input/day01").expect("Failed to read input");

    let entries: Vec<u32> = input
        .lines()
        .map(|line| line.parse().expect("Not a number"))
        .collect();

    println!("Part 1:\n{}", part1(&entries));
    println!("Part 2:\n{}", part2(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 241861950);
    }
}
