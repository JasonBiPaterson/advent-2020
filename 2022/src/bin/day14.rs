use nom::{bytes::complete::tag, IResult, ToUsize};
use std::fs;

fn parse(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|s| {
            let result: IResult<&str, Vec<(u32, u32)>> = nom::multi::separated_list1(
                tag(" -> "),
                nom::sequence::separated_pair(
                    nom::character::complete::u32,
                    tag(","),
                    nom::character::complete::u32,
                ),
            )(s);
            result
                .unwrap()
                .1
                .iter()
                .map(|(x, y)| (x.to_usize(), y.to_usize()))
                .collect()
        })
        .collect()
}

fn drop_sand(grid: &[Vec<bool>], (x, y): (usize, usize)) -> Option<(usize, usize)> {
    let col = &grid[x]
        .iter()
        .enumerate()
        .skip(y)
        .skip_while(|&(_, b)| !b)
        .next();

    if let &Some((y_new, _)) = col {
        if !grid[x - 1][y_new] {
            Some((x - 1, y_new))
        } else if !grid[x + 1][y_new] {
            Some((x + 1, y_new))
        } else {
            Some((x, y_new - 1))
        }
    } else {
        None
    }
}

fn part1(paths: &[Vec<(usize, usize)>]) -> usize {
    let verticies = paths.concat();
    let x_min = verticies.iter().map(|(x, _)| x).min().unwrap();
    let x_max = verticies.iter().map(|(x, _)| x).max().unwrap();
    let y_max = verticies.iter().map(|(_, y)| y).max().unwrap();

    let x_offset = x_min - 1;
    let mut grid = vec![vec![false; y_max + 1]; x_max - x_offset + 2];

    for path in paths {
        for line in path.windows(2) {
            let (x1, y1) = line[0];
            let (x2, y2) = line[1];
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[x - x_offset][y] = true;
                }
            }
        }
    }

    let mut grains = 0;
    'l: loop {
        let mut maybe_pos = Some((500 - x_offset, 0));
        while let Some(pos) = maybe_pos {
            match drop_sand(&grid, pos) {
                Some(new_pos) if new_pos == pos => {
                    grid[pos.0][pos.1] = true;
                    grains += 1;
                    maybe_pos = None
                }
                Some(new_pos) => maybe_pos = Some(new_pos),
                None => {
                    break 'l;
                }
            }
        }
    }

    grains
}

fn part2(paths: &[Vec<(usize, usize)>]) -> usize {
    let verticies = paths.concat();
    let y_max = verticies.iter().map(|(_, y)| y).max().unwrap();

    let x_offset = 500 - y_max - 2;
    let mut grid = vec![vec![false; y_max + 3]; 504 + 2* y_max];

    for path in paths {
        for line in path.windows(2) {
            let (x1, y1) = line[0];
            let (x2, y2) = line[1];
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[x - x_offset][y] = true;
                }
            }
        }
    }

    for x in 0..grid.len() {
        grid[x][y_max + 2] = true;
    }

    let mut grains = 0;
    'l: loop {
        let mut maybe_pos = Some((500 - x_offset, 0));
        while let Some(pos) = maybe_pos {
            match drop_sand(&grid, pos) {
                Some(new_pos) if new_pos == pos => {
                    grains += 1;
                    if pos == (500 - x_offset, 0) {
                        break 'l;
                    }
                    grid[pos.0][pos.1] = true;
                    maybe_pos = None;
                }
                Some(new_pos) => maybe_pos = Some(new_pos),
                None => {
                    break 'l;
                }
            }
        }
    }

    grains
}

fn main() {
    let input: String = fs::read_to_string("input/day14").expect("Failed to read input");
    let paths = parse(&input);

    println!("Part 1: {}", part1(&paths));
    println!("Part 2: {}", part2(&paths));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&TEST_INPUT)), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&TEST_INPUT)), 93);
    }
}
