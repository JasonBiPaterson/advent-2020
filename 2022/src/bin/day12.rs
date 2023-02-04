use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(grid: &[Vec<char>]) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, cs) in grid.iter().enumerate() {
        for (j, c) in cs.iter().enumerate() {
            if *c == 'S' {
                start = (i, j);
            } else if *c == 'E' {
                end = (i, j);
            }
        }
    }

    let x_max = grid.len() - 1;
    let y_max = grid[0].len() - 1;
    fn is_reachable(
        grid: &[Vec<char>],
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    ) -> bool {
        let c1 = grid[x1][y1];
        let c2 = grid[x2][y2];
        if c1 == 'S' {
            'a'.to_digit(36).unwrap() >= c2.to_digit(36).unwrap() - 1
        } else if c2 == 'E' {
            c1.to_digit(36).unwrap() >= 'z'.to_digit(36).unwrap() - 1
        } else {
            c1.to_digit(36).unwrap() >= c2.to_digit(36).unwrap() - 1
        }
    }

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parents = HashMap::new();

    q.push_back(start);
    visited.insert(start);

    while let Some(v) = q.pop_front() {
        if v == end {
            break;
        }

        let (x, y) = v;
        let mut es = vec![];
        if x > 0 && is_reachable(grid, (x, y), (x - 1, y)) {
            es.push((x - 1, y));
        }
        if y > 0 && is_reachable(grid, (x, y), (x, y - 1)) {
            es.push((x, y - 1));
        }
        if x < x_max && is_reachable(grid, (x, y), (x + 1, y)) {
            es.push((x + 1, y));
        }
        if y < y_max && is_reachable(grid, (x, y), (x, y + 1)) {
            es.push((x, y + 1));
        }

        for w in es {
            if !visited.contains(&w) {
                visited.insert(w);
                parents.insert(w, v);
                q.push_back(w);
            }
        }
    }

    let mut a = end;
    let mut ans = 0;
    while let Some(&p) = parents.get(&a) {
        ans += 1;
        a = p;
    }
    ans
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, cs) in grid.iter().enumerate() {
        for (j, c) in cs.iter().enumerate() {
            if *c == 'E' {
                end = (i, j);
            }
        }
    }

    let x_max = grid.len() - 1;
    let y_max = grid[0].len() - 1;
    fn is_reachable(
        grid: &[Vec<char>],
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    ) -> bool {
        let c1 = grid[x1][y1];
        let c2 = grid[x2][y2];
        if c1 == 'S' {
            'a'.to_digit(36).unwrap() >= c2.to_digit(36).unwrap() - 1
        } else if c2 == 'E' {
            c1.to_digit(36).unwrap() >= 'z'.to_digit(36).unwrap() - 1
        } else {
            c1.to_digit(36).unwrap() >= c2.to_digit(36).unwrap() - 1
        }
    }

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parents = HashMap::new();

    q.push_back(end);
    visited.insert(end);

    while let Some(v) = q.pop_front() {
        let (x, y) = v;

        if grid[x][y] == 'a' {
            start = v;
            break;
        }

        let mut es = vec![];
        if x > 0 && is_reachable(grid, (x - 1, y), (x, y)) {
            es.push((x - 1, y));
        }
        if y > 0 && is_reachable(grid, (x, y - 1), (x, y)) {
            es.push((x, y - 1));
        }
        if x < x_max && is_reachable(grid, (x + 1, y), (x, y)) {
            es.push((x + 1, y));
        }
        if y < y_max && is_reachable(grid, (x, y + 1), (x, y)) {
            es.push((x, y + 1));
        }

        for w in es {
            if !visited.contains(&w) {
                visited.insert(w);
                parents.insert(w, v);
                q.push_back(w);
            }
        }
    }

    let mut a = start;
    let mut ans = 0;
    while let Some(&p) = parents.get(&a) {
        ans += 1;
        a = p;
    }
    ans
}

fn main() {
    let input: String = fs::read_to_string("input/day12").expect("Failed to read input");
    let motions = parse(&input);

    println!("Part 1: {}", part1(&motions));
    println!("Part 2: {}", part2(&motions));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&TEST_INPUT)), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&TEST_INPUT)), 29);
    }
}

