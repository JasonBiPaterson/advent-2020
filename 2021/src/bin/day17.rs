use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn parse_input(input: &str) -> TargetArea {
    let re = Regex::new(r"x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = re.captures(input.trim()).unwrap();
    let parse_capture = |n: usize| -> i32 { captures.get(n).unwrap().as_str().parse().unwrap() };
    TargetArea {
        x_min: parse_capture(1),
        x_max: parse_capture(2),
        y_min: parse_capture(3),
        y_max: parse_capture(4),
    }
}

type InitialVels = HashMap<i32, Vec<i32>>;

fn triangle(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn triangle_rev(n: i32) -> i32 {
    (((2.0 * f64::from(n)) + 0.25).sqrt() - 0.5).ceil() as i32
}

fn part1(target: &TargetArea) -> i32 {
    let n = target.y_min.abs() - 1;
    triangle(n)
}

fn part2(target: &TargetArea) -> usize {
    let y_range = target.y_min..target.y_min.abs();
    let x_range = triangle_rev(target.x_min)..=target.x_max;

    let mut y_vels: InitialVels = HashMap::new();
    for y in y_range {
        let mut pos = 0;
        let mut vel = if y > 0 { -y - 1 } else { y };
        let mut i = if y > 0 { 2 * y + 1 } else { 0 };
        while pos >= target.y_min {
            if pos <= target.y_max {
                y_vels.entry(i).or_insert_with(Vec::new).push(y);
            }
            i += 1;
            pos += vel;
            vel -= 1;
        }
    }

    let mut points = HashSet::new();

    for x in x_range {
        for (&step, ys) in &y_vels {
            let tri = triangle(x);
            if step < x {
                let pos = tri - triangle(x - step);
                if pos >= target.x_min && pos <= target.x_max {
                    for &y in ys {
                        points.insert((x, y));
                    }
                }
            } else if tri <= target.x_max {
                for &y in ys {
                    points.insert((x, y));
                }
            }
        }
    }

    points.len()
}

fn main() {
    let input: String = fs::read_to_string("input/day17").expect("Failed to read input");

    let target = parse_input(&input);

    println!("Part 1: {}", part1(&target));
    println!("Part 2: {}", part2(&target));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        let target = parse_input(INPUT);
        assert_eq!(part1(&target), 45);
    }

    #[test]
    fn test_part2() {
        let target = parse_input(INPUT);
        assert_eq!(part2(&target), 112);
    }
}
