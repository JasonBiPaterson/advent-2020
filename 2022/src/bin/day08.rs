use std::{collections::HashSet, fs};

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(tree_map: &[Vec<u32>]) -> usize {
    let n = tree_map.len();
    let m = tree_map[0].len();

    let mut visible_trees = HashSet::new();
    visible_trees.insert((0, 0));
    visible_trees.insert((0, m - 1));
    visible_trees.insert((n - 1, 0));
    visible_trees.insert((n - 1, m - 1));

    #[allow(clippy::needless_range_loop)]
    for i in 1..n - 1 {
        visible_trees.insert((i, 0));
        let mut prev_tree = tree_map[i][0];
        for j in 1..m - 1 {
            let tree = tree_map[i][j];
            if prev_tree < tree {
                visible_trees.insert((i, j));
                prev_tree = tree;
            }
        }

        visible_trees.insert((i, m - 1));
        let mut prev_tree = tree_map[i][m - 1];
        for j in (1..m - 1).rev() {
            let tree = tree_map[i][j];
            if prev_tree < tree {
                visible_trees.insert((i, j));
                prev_tree = tree;
            }
        }
    }

    for j in 1..m - 1 {
        visible_trees.insert((0, j));
        let mut prev_tree = tree_map[0][j];
        #[allow(clippy::needless_range_loop)]
        for i in 1..n - 1 {
            let tree = tree_map[i][j];
            if prev_tree < tree {
                visible_trees.insert((i, j));
                prev_tree = tree;
            }
        }

        visible_trees.insert((n - 1, j));
        let mut prev_tree = tree_map[n - 1][j];
        for i in (1..n - 1).rev() {
            let tree = tree_map[i][j];
            if prev_tree < tree {
                visible_trees.insert((i, j));
                prev_tree = tree;
            }
        }
    }
    visible_trees.len()
}

fn part2(tree_map: &[Vec<u32>]) -> u32 {
    let n = tree_map.len();
    let m = tree_map[0].len();

    let mut max_view = 0;
    for (i, tree_row) in tree_map.iter().enumerate() {
        for (j, tree) in tree_row.iter().enumerate() {
            let mut view_score: u32 = 1;

            let mut partial_score: u32 = 0;
            #[allow(clippy::needless_range_loop)]
            for i_2 in i + 1..n {
                partial_score += 1;
                if tree_map[i_2][j] >= *tree {
                    break;
                }
            }
            view_score *= partial_score;

            let mut partial_score: u32 = 0;
            for i_2 in (0..i).rev() {
                partial_score += 1;
                if tree_map[i_2][j] >= *tree {
                    break;
                }
            }
            view_score *= partial_score;

            let mut partial_score: u32 = 0;
            for j_2 in j + 1..m {
                partial_score += 1;
                if tree_map[i][j_2] >= *tree {
                    break;
                }
            }
            view_score *= partial_score;

            let mut partial_score: u32 = 0;
            for j_2 in (0..j).rev() {
                partial_score += 1;
                if tree_map[i][j_2] >= *tree {
                    break;
                }
            }
            view_score *= partial_score;

            if view_score >= max_view {
                max_view = view_score;
            }
        }
    }
    max_view
}

fn main() {
    let input: String = fs::read_to_string("input/day08").expect("Failed to read input");
    let tree_map = parse(&input);

    println!("Part 1: {}", part1(&tree_map));
    println!("Part 2: {}", part2(&tree_map));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 8);
    }
}
