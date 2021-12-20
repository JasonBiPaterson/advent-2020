use std::{collections::HashSet, fs};

type ParsedInput = Image;

#[derive(Debug, Clone)]
struct Image {
    algorithm: Vec<bool>,
    background: bool,
    pixels: HashSet<(isize, isize)>,
}

impl Image {
    fn range(&self) -> (isize, isize, isize, isize) {
        let min_x = self.pixels.iter().map(|p| p.0).min().unwrap();
        let max_x = self.pixels.iter().map(|p| p.0).max().unwrap();
        let min_y = self.pixels.iter().map(|p| p.1).min().unwrap();
        let max_y = self.pixels.iter().map(|p| p.1).max().unwrap();
        (min_x, max_x, min_y, max_y)
    }

    fn enhance(&mut self) {
        let (min_x, max_x, min_y, max_y) = self.range();

        let mut next_image = HashSet::new();

        for i in (min_x - 1)..=(max_x + 1) {
            for j in (min_y - 1)..=(max_y + 1) {
                let index = [
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ]
                .iter()
                .map(|t| {
                        if t.0 < min_x || t.0 > max_x || t.1 < min_y || t.1 > max_y
                        { return self.background }
                        self.pixels.get(t).is_some()
                })
                .fold(0, |n, b| (n << 1) | b as usize);

                if self.algorithm[index] {
                    next_image.insert((i, j));
                };
            }
        }
        self.pixels = next_image;
        self.background = if self.background {
            self.algorithm[511]
        } else {
            self.algorithm[0]
        };
    }

    fn print(&self) {
        let (min_x, max_x, min_y, max_y) = self.range();

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                if self.pixels.get(&(i, j)).is_some() {
                    print!("{}", if self.background { "." } else { "#" });
                } else {
                    print!("{}", if self.background { "#" } else { "." });
                };
            }
            println!();
        }
        println!();
    }
}

fn parse_input(input: &str) -> Image {
    let (algo_input, image_input) = input.trim().split_once("\n\n").unwrap();

    let algorithm = algo_input.trim().chars().map(|ch| ch == '#').collect();

    let mut image = HashSet::new();

    for (i, line) in image_input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                image.insert((i as isize, j as isize));
            } else if ch != '.' {
                panic!()
            }
        }
    }

    Image {
        algorithm,
        background: false,
        pixels: image,
    }
}

fn part1(parsed_input: &mut ParsedInput) -> usize {
    for _ in 0..2 {
        parsed_input.enhance();
    }
    parsed_input.pixels.len()
}

fn part2(parsed_input: &mut ParsedInput) -> usize {
    for _ in 0..50 {
        parsed_input.enhance();
    }
    parsed_input.pixels.len()
}

fn main() {
    let input: String = fs::read_to_string("input/day20").expect("Failed to read input");

    let mut parsed_input = parse_input(&input);

    println!("Part 1: {}", part1(&mut parsed_input.clone()));
    println!("Part 2: {}", part2(&mut parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn test_part1() {
        let mut parsed_input = parse_input(INPUT);
        assert_eq!(part1(&mut parsed_input), 35);
        parsed_input.print();
    }

    #[test]
    fn test_part2() {
        let mut parsed_input = parse_input(INPUT);
        assert_eq!(part2(&mut parsed_input), 3351);
    }
}
