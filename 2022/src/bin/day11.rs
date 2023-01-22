use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: (Option<u64>, fn(u64, u64) -> u64, Option<u64>),
    div_test: u64,
    if_true: usize,
    if_false: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for line in input.lines().collect::<Vec<&str>>().chunks(7) {
        let starting_items = line[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>();

        let op_tmp = line[2]
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .splitn(3, " ")
            .collect::<Vec<&str>>();
        let operation = (
            op_tmp[0].parse::<u64>().ok(),
            match op_tmp[1] {
                "*" => std::ops::Mul::mul,
                "+" => std::ops::Add::add,
                _ => unreachable!(),
            },
            op_tmp[2].parse::<u64>().ok(),
        );

        let divisibility_test = line[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let if_true = line[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let if_false = line[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            items: starting_items,
            operation,
            div_test: divisibility_test,
            if_true,
            if_false,
        });
    }
    monkeys
}

fn part1(monkeys: &[Monkey]) -> u64 {
    let mut asdf: Vec<(Vec<u64>, u64)> = monkeys
        .iter()
        .map(|monkey| (monkey.items.clone(), 0))
        .collect();

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in asdf[i].clone().0 {
                let new_item = (monkey.operation.1)(
                    monkey.operation.0.unwrap_or(item),
                    monkey.operation.2.unwrap_or(item),
                ) / 3;
                if new_item % monkey.div_test == 0 {
                    asdf[monkey.if_true].0.push(new_item);
                } else {
                    asdf[monkey.if_false].0.push(new_item);
                }
                asdf[i].1 += 1;
            }
            asdf[i].0 = vec![];
        }
    }
    asdf.sort_by(|a, b| b.1.cmp(&a.1));
    asdf[0].1 * asdf[1].1
}

fn part2(monkeys: &[Monkey]) -> u64 {
    let mut asdf: Vec<(Vec<u64>, u64)> = monkeys
        .iter()
        .map(|monkey| (monkey.items.clone(), 0))
        .collect();

    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.div_test)
        .reduce(std::ops::Mul::mul)
        .unwrap();

    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in asdf[i].clone().0 {
                let new_item = (monkey.operation.1)(
                    monkey.operation.0.unwrap_or(item),
                    monkey.operation.2.unwrap_or(item),
                ) % modulo;
                if new_item % monkey.div_test == 0 {
                    asdf[monkey.if_true].0.push(new_item);
                } else {
                    asdf[monkey.if_false].0.push(new_item);
                }
                asdf[i].1 += 1;
            }
            asdf[i].0 = vec![];
        }
    }
    asdf.sort_by(|a, b| b.1.cmp(&a.1));
    asdf[0].1 * asdf[1].1
}

fn main() {
    let input: String = fs::read_to_string("input/day11").expect("Failed to read input");
    let motions = parse(&input);

    println!("Part 1: {}", part1(&motions));
    println!("Part 2: {}", part2(&motions));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&TEST_INPUT)), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&TEST_INPUT)), 2713310158);
    }
}
