use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn flash_window(i: usize, i_max: usize) -> std::ops::RangeInclusive<usize> {
    if i == 0 {
        i..=1
    } else if i == i_max {
        (i - 1)..=i_max
    } else {
        (i - 1)..=(i + 1)
    }
}

fn step(mut octopodes: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let i_max = octopodes.len() - 1;
    let j_max = octopodes[0].len() - 1;
    for row in &mut octopodes {
        for octopus in row {
            *octopus += 1;
        }
    }
    loop {
        let mut flashes = 0_u32;
        for i in 0..=i_max {
            for j in 0..=j_max {
                if octopodes[i][j] > 9 {
                    octopodes[i][j] = -1;
                    flashes += 1;
                    for ii in flash_window(i, i_max) {
                        for jj in flash_window(j, j_max) {
                            if octopodes[ii][jj] != -1 {
                                octopodes[ii][jj] += 1;
                            };
                        }
                    }
                }
            }
        }
        if flashes == 0 {
            break;
        }
    }
    for row in &mut octopodes {
        for octopus in row {
            if octopus == &-1 {
                *octopus = 0;
            }
        }
    }
    octopodes
}

fn part1(mut octopodes: Vec<Vec<i8>>) -> u32 {
    let mut flashes = 0_u32;
    for _ in 0..100 {
        octopodes = step(octopodes);
        for row in &octopodes {
            for octopus in row {
                if octopus == &0 {
                    flashes += 1;
                }
            }
        }
    }
    flashes
}

fn part2(mut octopodes: Vec<Vec<i8>>) -> u32 {
    let mut count = 0_u32;
    loop {
        octopodes = step(octopodes);
        count += 1;
        if octopodes.iter().flatten().all(|&octopus| { octopus == 0 }) {
            break;
        }
    }
    count
}

fn main() {
    let input: String = fs::read_to_string("input/day11").expect("Failed to read input");

    let octopodes = parse_input(&input);

    println!("Part 1: {}", part1(octopodes.clone()));
    println!("Part 2: {}", part2(octopodes));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_part1() {
        let octopodes = parse_input(INPUT);
        assert_eq!(part1(octopodes), 1656);
    }

    #[test]
    fn test_part2() {
        let octopodes = parse_input(INPUT);
        assert_eq!(part2(octopodes), 195);
    }
}
