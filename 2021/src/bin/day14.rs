use std::{collections::HashMap, fs};

type InsertionRules = HashMap<(char, char), char>;
type ParsedInput = (char, Vec<(char, char)>, InsertionRules);
type PairCount = HashMap<(char, char), u64>;

fn parse_input(input: &str) -> ParsedInput {
    let (polymer_template, insertion_rule_input) = input.trim().split_once("\n\n").unwrap();

    let last_char = polymer_template.chars().last().unwrap();
    let char_pairs = polymer_template
        .chars()
        .zip(polymer_template.chars().skip(1))
        .collect();

    let mut insertion_rules = HashMap::new();
    for line in insertion_rule_input.lines() {
        let mut chars = line.chars();
        let k = (chars.next().unwrap(), chars.next().unwrap());
        let v = chars.nth(4).unwrap();
        insertion_rules.insert(k, v);
    }
    (last_char, char_pairs, insertion_rules)
}

fn polymerize((last_char, char_pairs, insertion_rules): &ParsedInput, steps: u32) -> u64 {
    let mut pair_count: PairCount = HashMap::new();
    for &pair in char_pairs {
        *pair_count.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut next_pair_count: PairCount = HashMap::new();
        for (&pair, &count) in &pair_count {
            let &ch = insertion_rules.get(&pair).unwrap();
            *next_pair_count.entry((pair.0, ch)).or_insert(0) += count;
            *next_pair_count.entry((ch, pair.1)).or_insert(0) += count;
        }
        pair_count = next_pair_count;
    }

    let mut char_count = HashMap::new();
    for (&pair, &count) in &pair_count {
        *char_count.entry(pair.0).or_insert(0) += count;
    }
    *char_count.entry(*last_char).or_insert(0) += 1;

    let max = *char_count.iter().max_by_key(|x| x.1).unwrap().1;
    let min = *char_count.iter().min_by_key(|x| x.1).unwrap().1;
    max - min
}

fn main() {
    let input: String = fs::read_to_string("input/day14").expect("Failed to read input");

    let parsed_input = parse_input(&input);

    println!("Part 1: {}", polymerize(&parsed_input, 10));
    println!("Part 2: {}", polymerize(&parsed_input, 40));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_part1() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(polymerize(&parsed_input, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(polymerize(&parsed_input, 40), 2_188_189_693_529);
    }
}
