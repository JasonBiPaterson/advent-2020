use std::fs;

type Board = Vec<Vec<u32>>;

fn bingo_score(draws: &[u32], board: &[Vec<u32>]) -> Option<u32> {
    let is_bingo = (0..5).any(|i| {
        (0..5).all(|j| draws.contains(&board[i][j])) || (0..5).all(|j| draws.contains(&board[j][i]))
    });
    match is_bingo {
        true => Some(
            board
                .iter()
                .flatten()
                .filter(|&n| !draws.contains(n))
                .sum::<u32>(),
        ),
        false => None,
    }
}

fn part1(draws: &[u32], boards: &[Board]) -> u32 {
    for i in 5..draws.len() {
        let draw_slice = &draws[0..i];
        let score = boards
            .iter()
            .find_map(|board| bingo_score(draw_slice, board));
        if let Some(..) = score {
            return score.unwrap() * draw_slice.last().unwrap();
        }
    }
    panic!("No bingo!")
}

fn part2(draws: &[u32], boards: &[Board]) -> u32 {
    let mut remaining_boards = boards.to_vec();
    for i in 5..draws.len() {
        let draw_slice = &draws[0..i];
        if remaining_boards.len() == 1 {
            let score = bingo_score(draw_slice, &remaining_boards[0]);
            if let Some(..) = score {
                return score.unwrap() * draw_slice.last().unwrap();
            }
        } else {
            remaining_boards.retain(|board| bingo_score(draw_slice, board) == None);
        };
    }
    panic!("No bingo!")
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.trim().split("\n\n");

    let draws: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards: Vec<Board> = lines
        .map(|board| {
            board
                .lines()
                .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
                .collect()
        })
        .collect();

    (draws, boards)
}

fn main() {
    let input: String = fs::read_to_string("input/day04").expect("Failed to read input");

    let (draws, boards) = parse_input(&input);

    println!("Part 1: {}", part1(&draws, &boards));
    println!("Part 2: {}", part2(&draws, &boards));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_part1() {
        let (draws, boards) = parse_input(INPUT);
        assert_eq!(part1(&draws, &boards), 4512);
    }

    #[test]
    fn test_part2() {
        let (draws, boards) = parse_input(INPUT);
        assert_eq!(part2(&draws, &boards), 1924);
    }
}
