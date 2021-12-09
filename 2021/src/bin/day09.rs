use std::collections::HashSet;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn low_points(caves: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_points: Vec<(usize, usize)> = vec![];
    let n1 = caves.len();
    let n2 = caves[0].len();
    caves.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, &v)| {
            if { i == 0 || caves[i - 1][j] > v }
                && { i == n1 - 1 || caves[i + 1][j] > v }
                && { j == 0 || caves[i][j - 1] > v }
                && { j == n2 - 1 || caves[i][j + 1] > v }
            {
                low_points.push((i, j));
            }
        });
    });
    low_points
}

fn part1(caves: &[Vec<u32>]) -> u32 {
    low_points(caves)
        .iter()
        .map(|point| caves[point.0][point.1] + 1)
        .sum()
}

fn part2(caves: &[Vec<u32>]) -> usize {
    let low_points = low_points(caves);
    let n1 = caves.len();
    let n2 = caves[0].len();
    let mut basin_sizes: Vec<usize> = vec![];
    for low_point in low_points {
        let mut basins = HashSet::new();
        basins.insert(low_point);
        loop {
            let mut next_basins = basins.clone();
            for point in &basins {
                let i = point.0;
                let j = point.1;
                if i != 0 && caves[i - 1][j] < 9 {
                    next_basins.insert((i - 1, j));
                }
                if i != n1 - 1 && caves[i + 1][j] < 9 {
                    next_basins.insert((i + 1, j));
                }
                if j != 0 && caves[i][j - 1] < 9 {
                    next_basins.insert((i, j - 1));
                }
                if j != n2 - 1 && caves[i][j + 1] < 9 {
                    next_basins.insert((i, j + 1));
                }
            }
            if next_basins != basins {
                basins = next_basins;
                continue;
            }
            break;
        }
        basin_sizes.push(basins.len());
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let input: String = fs::read_to_string("input/day09").expect("Failed to read input");

    let caves = parse_input(&input);

    println!("Part 1: {}", part1(&caves));
    println!("Part 2: {}", part2(&caves));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_part1() {
        let entries = parse_input(INPUT);
        assert_eq!(part1(&entries), 15);
    }

    #[test]
    fn test_part2() {
        let entries = parse_input(INPUT);
        assert_eq!(part2(&entries), 1134);
    }
}
