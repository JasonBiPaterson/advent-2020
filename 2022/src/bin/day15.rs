use nom::{bytes::complete::tag, IResult};
use std::fs;

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

fn parse(input: &str) -> Vec<(Position, Position)> {
    input
        .lines()
        .map(|s| {
            let result: IResult<_, _> = nom::sequence::tuple((
                tag("Sensor at x="),
                nom::character::complete::i64,
                tag(", y="),
                nom::character::complete::i64,
                tag(": closest beacon is at x="),
                nom::character::complete::i64,
                tag(", y="),
                nom::character::complete::i64,
            ))(s);
            let (_, sensor_x, _, sonsor_y, _, beacon_x, _, beacon_y) = result.unwrap().1;
            (
                Position {
                    x: sensor_x,
                    y: sonsor_y,
                },
                Position {
                    x: beacon_x,
                    y: beacon_y,
                },
            )
        })
        .collect()
}

fn bad_ranges(report: &[(Position, Position)], y: i64) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();

    for (sensor, beacon) in report {
        let radius = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        let x_diff = radius - (sensor.y - y).abs();

        if x_diff >= 0 {
            ranges.push(((sensor.x - x_diff), (sensor.x + x_diff)));
        }
    }

    for i in 0..ranges.len() {
        for j in 0..ranges.len() {
            let a = ranges[i];
            let b = ranges[j];
            if (a.0 <= b.0 && b.0 <= a.1 + 1) || (a.0 - 1 <= b.1 && b.1 <= a.1) {
                ranges[j] = (a.0.min(b.0), a.1.max(b.1))
            }
        }
    }
    ranges.sort();
    ranges.dedup();
    ranges
}

fn part1(report: &[(Position, Position)], y: i64) -> i64 {
    let merged_ranges = bad_ranges(report, y);

    let mut beacon_xs: Vec<i64> = report
        .iter()
        .filter(|&(_, beacon)| beacon.y == y)
        .map(|(_, beacon)| beacon.x)
        .collect();
    beacon_xs.sort();
    beacon_xs.dedup();

    let mut ans = 0;
    for range in merged_ranges {
        ans += range.1 - range.0 + 1;
        for x in &beacon_xs {
            if &range.0 <= x && x <= &range.1 {
                ans -= 1;
            }
        }
    }
    ans
}

fn part2(report: &[(Position, Position)], search_max: i64) -> i64 {
    for y in 0..=search_max {
        let ranges = bad_ranges(report, y);
        if ranges.len() > 1 {
            let x = ranges[0].1 + 1;
            return 4000000 * x + y;
        }
    }
    0
}

fn main() {
    let input: String = fs::read_to_string("input/day15").expect("Failed to read input");
    let report = parse(&input);

    println!("Part 1: {}", part1(&report, 2000000));
    println!("Part 2: {}", part2(&report, 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&TEST_INPUT), 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&TEST_INPUT), 20), 56000011);
    }
}
