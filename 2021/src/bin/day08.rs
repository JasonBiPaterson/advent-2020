use std::fs;

type Entry<'a> = (Vec<&'a str>, Vec<&'a str>);

fn order_len(s: &str) -> usize {
    match s.chars().count() {
        2 | 4 | 3 | 7 => 0, // 1 | 4 | 7 | 8
        6 => 1,             // 0 | 6 | 9
        5 => 2,             // 2 | 3 | 5
        _ => panic!(),
    }
}

fn is_superset(s1: &str, s2: &str) -> bool {
    s1.chars().all(|ch| s2.contains(ch))
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .trim()
        .lines()
        .map(|line| line.split_once(" | "))
        .map(|a| match a {
            None => panic!(),
            Some((signal_pattern, output_value)) => (
                signal_pattern.split_whitespace().collect(),
                output_value.split_whitespace().collect(),
            ),
        })
        .collect()
}

fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|t| &t.1)
        .filter(|s| matches!(s.chars().count(), 2 | 4 | 3 | 7))
        .count()
}

fn part2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|(signal_pattern, output_value): &Entry| {
            let mut sp = signal_pattern.clone();
            sp.sort_by(|&a, &b| order_len(a).cmp(&order_len(b)));
            let mut patterns = [""; 10];
            for s in sp {
                match s.chars().count() {
                    2 => patterns[1] = s,
                    4 => patterns[4] = s,
                    3 => patterns[7] = s,
                    7 => patterns[8] = s,
                    6 if !is_superset(patterns[1], s) => patterns[6] = s,
                    6 if is_superset(patterns[4], s) => patterns[9] = s,
                    6 => patterns[0] = s,
                    5 if is_superset(s, patterns[6]) => patterns[5] = s,
                    5 if is_superset(patterns[1], s) => patterns[3] = s,
                    5 => patterns[2] = s,
                    _ => panic!(),
                }
            }

            output_value
                .iter()
                .enumerate()
                .map(|(i, &ov)| -> usize {
                    patterns
                        .iter()
                        .enumerate()
                        .find_map(|(j, &s)| {
                            if is_superset(ov, s) && is_superset(s, ov) {
                                Some(j * (10_usize.pow((3 - i).try_into().unwrap())))
                            } else {
                                None
                            }
                        })
                        .unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input: String = fs::read_to_string("input/day08").expect("Failed to read input");

    let entries = parse_input(&input);

    println!("Part 1: {}", part1(&entries));
    println!("Part 2: {}", part2(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part1() {
        let entries = parse_input(INPUT);
        assert_eq!(part1(&entries), 26);
    }

    #[test]
    fn test_part2() {
        let entries = parse_input(INPUT);
        assert_eq!(part2(&entries), 61229);
    }
}
