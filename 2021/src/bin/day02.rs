use std::fs;

fn part1(words: &Vec<&str>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for line in words.chunks_exact(2) {
        let direction = line[0];
        let distance: i32 = line[1].parse().expect("NaN");
        match direction {
            "forward" => x += distance,
            "up" => y -= distance,
            "down" => y += distance,
            _ => panic!("Not a valid direction"),
        };
    }
    x * y
}

fn part2(words: &Vec<&str>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in words.chunks_exact(2) {
        let direction = line[0];
        let distance: i32 = line[1].parse().expect("NaN");
        match direction {
            "forward" => {
                x += distance;
                y += distance * aim;
            }
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => panic!("Not a valid direction"),
        };
    }
    x * y
}

fn main() {
    let input: String = fs::read_to_string("input/day02").expect("Failed to read input");

    let words: Vec<&str> = input.trim().split_whitespace().collect();

    println!("Part 1: {}", part1(&words));
    println!("Part 2: {}", part2(&words));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let words = vec![
            "forward", "5", "down", "5", "forward", "8", "up", "3", "down", "8", "forward", "2",
        ];
        assert_eq!(part1(&words), 150);
    }

    #[test]
    fn test_part2() {
        let words = vec![
            "forward", "5", "down", "5", "forward", "8", "up", "3", "down", "8", "forward", "2",
        ];
        assert_eq!(part2(&words), 900);
    }
}
