use std::collections::HashMap;
use std::{cmp, fs};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    point1: Point,
    point2: Point,
}

impl Line {
    fn is_cardinal(&self) -> bool {
        self.point1.x == self.point2.x || self.point1.y == self.point2.y
    }

    fn arrange_ends(&self) -> (&Point, &Point) {
        match self.point1.x.cmp(&self.point2.x) {
            cmp::Ordering::Less => (&self.point1, &self.point2),
            cmp::Ordering::Equal if self.point1.y < self.point2.y => (&self.point1, &self.point2),
            cmp::Ordering::Equal | cmp::Ordering::Greater => (&self.point2, &self.point1),
        }
    }

    fn to_points(&self) -> Vec<Point> {
        let (p1, p2) = self.arrange_ends();
        if p1.x == p2.x {
            (p1.y..=p2.y).map(|y| Point { x: p1.x, y }).collect()
        } else if p1.y == p2.y {
            (p1.x..=p2.x).map(|x| Point { x, y: p1.y }).collect()
        } else if p1.y < p2.y {
            (p1.x..=p2.x)
                .zip(p1.y..=p2.y)
                .map(|(x, y)| Point { x, y })
                .collect()
        } else {
            (p1.x..=p2.x)
                .zip((p2.y..=p1.y).rev())
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(|c| !char::is_numeric(c))
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|line| Line {
            point1: Point {
                x: line[0],
                y: line[1],
            },
            point2: Point {
                x: line[2],
                y: line[3],
            },
        })
        .collect::<Vec<Line>>()
}

fn part1(lines: &[Line]) -> usize {
    let mut point_count = HashMap::new();

    lines
        .iter()
        .filter(|&line| line.is_cardinal())
        .flat_map(Line::to_points)
        .for_each(|point| {
            *point_count.entry(point).or_insert(0_usize) += 1;
        });

    point_count.values().filter(|&n| n >= &2).count()
}

fn part2(lines: &[Line]) -> usize {
    let mut point_count = HashMap::new();

    lines.iter().flat_map(Line::to_points).for_each(|point| {
        *point_count.entry(point).or_insert(0_usize) += 1;
    });

    point_count.values().filter(|&n| n >= &2).count()
}

fn main() {
    let input: String = fs::read_to_string("input/day05").expect("Failed to read input");

    let lines = parse_input(&input);

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn test_part1() {
        let lines = parse_input(INPUT);
        assert_eq!(part1(&lines), 5);
    }

    #[test]
    fn test_part2() {
        let lines = parse_input(INPUT);
        assert_eq!(part2(&lines), 12);
    }
}
