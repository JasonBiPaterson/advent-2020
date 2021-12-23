use aoc2021::dijkstra;
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.x.cmp(&self.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Cavern = Vec<Vec<usize>>;
type RefCavern<'a> = &'a [Vec<usize>];

fn parse_input(input: &str) -> Cavern {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn total_risk(cavern: RefCavern) -> usize {
    let goal = (cavern[0].len() - 1, cavern.len() - 1);
    dijkstra::shortest_path(
        &|Position { x, y }| {
            let mut adj: Vec<dijkstra::Edge<Position>> = vec![];
            if x > 0 {
                adj.push(dijkstra::Edge {
                    cost: cavern[x - 1][y],
                    node: Position { x: x - 1, y },
                });
            }
            if y > 0 {
                adj.push(dijkstra::Edge {
                    cost: cavern[x][y - 1],
                    node: Position { x, y: y - 1 },
                });
            }
            if x < goal.0 {
                adj.push(dijkstra::Edge {
                    cost: cavern[x + 1][y],
                    node: Position { x: x + 1, y },
                });
            }
            if y < goal.1 {
                adj.push(dijkstra::Edge {
                    cost: cavern[x][y + 1],
                    node: Position { x, y: y + 1 },
                });
            }
            adj
        },
        Position { x: 0, y: 0 },
        Position {
            x: goal.0,
            y: goal.1,
        },
    )
    .unwrap()
}

fn part1(cavern: RefCavern) -> usize {
    total_risk(cavern)
}

fn part2(cavern: RefCavern) -> usize {
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
