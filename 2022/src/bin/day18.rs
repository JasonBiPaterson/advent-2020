use nom::bytes::complete::tag;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fs;

type Cube = (i8, i8, i8);

fn count_adjacent(cube_map: &mut HashMap<(i8, i8), Vec<i8>>) -> usize {
    let mut common_sides = 0;
    for v in cube_map.values_mut() {
        v.sort_unstable();
        for coll in v.windows(2) {
            if coll[0] + 1 == coll[1] {
                common_sides += 1;
            }
        }
    }
    cube_map.clear();
    common_sides
}

fn part1(cubes: &[Cube]) -> usize {
    let mut common_sides = 0;

    let mut cube_map = HashMap::new();
    for &(x, y, z) in cubes {
        let v = cube_map.entry((x, y)).or_insert_with(Vec::new);
        v.push(z);
    }
    common_sides += count_adjacent(&mut cube_map);
    for &(x, y, z) in cubes {
        let v = cube_map.entry((x, z)).or_insert_with(Vec::new);
        v.push(y);
    }
    common_sides += count_adjacent(&mut cube_map);
    for &(x, y, z) in cubes {
        let v = cube_map.entry((y, z)).or_insert_with(Vec::new);
        v.push(x);
    }
    common_sides += count_adjacent(&mut cube_map);

    6 * cubes.len() - 2 * common_sides
}

fn part2(cubes: &[Cube]) -> usize {
    let mut max_pos = (0, 0, 0);
    let mut min_pos = (127, 127, 127);
    for &cube in cubes {
        max_pos.0 = max_pos.0.max(cube.0);
        max_pos.1 = max_pos.1.max(cube.1);
        max_pos.2 = max_pos.2.max(cube.2);
        min_pos.0 = min_pos.0.min(cube.0);
        min_pos.1 = min_pos.1.min(cube.1);
        min_pos.2 = min_pos.2.min(cube.2);
    }
    max_pos.0 += 1;
    max_pos.1 += 1;
    max_pos.2 += 1;
    min_pos.0 -= 1;
    min_pos.1 -= 1;
    min_pos.2 -= 1;

    let mut points = HashSet::new();
    for i in min_pos.0..=max_pos.0 {
        for j in min_pos.1..=max_pos.1 {
            for k in min_pos.2..=max_pos.2 {
                points.insert((i, j, k));
            }
        }
    }
    points.remove(&min_pos);

    let mut steam_front = HashSet::from([min_pos]);

    while !steam_front.is_empty() {
        let mut adjacent_positions = HashSet::new();
        for pos in &steam_front {
            adjacent_positions.extend([
                (pos.0 - 1, pos.1, pos.2),
                (pos.0, pos.1 - 1, pos.2),
                (pos.0, pos.1, pos.2 - 1),
                (pos.0 + 1, pos.1, pos.2),
                (pos.0, pos.1 + 1, pos.2),
                (pos.0, pos.1, pos.2 + 1),
            ]);
        }

        adjacent_positions.retain(|pos| !cubes.contains(pos) && points.contains(pos));

        for &pos in &adjacent_positions {
            points.remove(&pos);
        }
        steam_front = adjacent_positions;
    }

    part1(&points.into_iter().collect::<Vec<_>>())
}

fn parse(input: &str) -> Vec<Cube> {
    let result: IResult<_, _> = nom::multi::separated_list0(
        tag("\n"),
        nom::combinator::map(
            nom::sequence::tuple((
                nom::character::complete::i8,
                tag(","),
                nom::character::complete::i8,
                tag(","),
                nom::character::complete::i8,
            )),
            |(x, _, y, _, z)| (x, y, z),
        ),
    )(input);
    result.unwrap().1
}

fn main() {
    let input: String = fs::read_to_string("input/day18").expect("Failed to read input");
    let cubes = parse(&input);

    println!("Part 1: {}", part1(&cubes));
    println!("Part 2: {}", part2(&cubes));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 58);
    }
}
