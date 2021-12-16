use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Cavern = Vec<Vec<u32>>;
type RefCavern<'a> = &'a [Vec<u32>];

fn parse_input(input: &str) -> Cavern {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn total_risk(cavern: RefCavern) -> u32 {
    let mut heap = BinaryHeap::new();
    let goal = (cavern[0].len() - 1, cavern.len() - 1);
    let mut dist: Cavern = (0..=goal.0)
        .map(|_| (0..=goal.1).map(|_| u32::MAX).collect())
        .collect();

    heap.push(State {
        x: 0,
        y: 0,
        cost: 0,
    });

    while let Some(State { x, y, cost }) = heap.pop() {
        if (x, y) == goal {
            return cost;
        }
        if cost > dist[x][y] {
            continue;
        }

        let mut adj_ceiling: Vec<(usize, usize)> = vec![];
        if x > 0 {
            adj_ceiling.push((x - 1, y));
        }
        if y > 0 {
            adj_ceiling.push((x, y - 1));
        }
        if x < goal.0 {
            adj_ceiling.push((x + 1, y));
        }
        if y < goal.1 {
            adj_ceiling.push((x, y + 1));
        }

        for &(a, b) in &adj_ceiling {
            let next = State {
                cost: cost + cavern[a][b],
                x: a,
                y: b,
            };

            if next.cost < dist[a][b] {
                heap.push(next);
                dist[a][b] = next.cost;
            }
        }
    }
    panic!()
}

fn part1(cavern: RefCavern) -> u32 {
    total_risk(cavern)
}

fn part2(cavern: RefCavern) -> u32 {
    let mut full_cavern: Cavern = vec![vec![]; cavern.len() * 5];
    for n in 0..5 {
        for m in 0..5 {
            for (i, row) in &mut cavern.iter().enumerate() {
                for ceiling in row {
                    full_cavern[i + cavern.len() * (m as usize)]
                        .push(((ceiling + n + m - 1) % 9) + 1);
                }
            }
        }
    }
    total_risk(&full_cavern)
}

fn main() {
    let input: String = fs::read_to_string("input/day15").expect("Failed to read input");

    let parsed_input = parse_input(&input);

    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part1() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(part1(&parsed_input), 40);
    }

    #[test]
    fn test_part2() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(part2(&parsed_input), 315);
    }
}
