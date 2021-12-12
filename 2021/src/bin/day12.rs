use std::collections::{HashMap, HashSet};
use std::fs;

fn search<'a>(
    cave: &'a str,
    graph: &HashMap<&str, Vec<&'a str>>,
    blocked_caves: &mut HashSet<&'a str>,
) -> u32 {
    if cave == "end" {
        return 1;
    }
    let mut count = 0;
    if cave.chars().all(char::is_lowercase) {
        blocked_caves.insert(cave);
    }
    for &next_cave in graph.get(cave).unwrap() {
        if blocked_caves.get(next_cave).is_none() {
            count += search(next_cave, graph, blocked_caves);
        }
    }
    blocked_caves.remove(cave);
    count
}

fn search2<'a>(
    cave: &'a str,
    graph: &HashMap<&str, Vec<&'a str>>,
    blocked_caves: &mut HashSet<&'a str>,
    double_visit: &mut Option<&'a str>,
) -> u32 {
    if cave == "end" {
        return 1;
    }
    let mut count = 0;
    if cave.chars().all(char::is_lowercase) && !blocked_caves.insert(cave) {
        *double_visit = Some(cave);
    }
    for &next_cave in graph.get(cave).unwrap() {
        if blocked_caves.get(next_cave).is_none() || double_visit.is_none() {
            count += search2(next_cave, graph, blocked_caves, double_visit);
        }
    }
    if Some(cave) == *double_visit {
        *double_visit = None;
    } else {
        blocked_caves.remove(cave);
    }
    count
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let pairs = input
        .trim()
        .lines()
        .map(|line| line.trim().split_once('-').unwrap());
    let mut graph = HashMap::new();
    for (a, b) in pairs {
        match (a, b) {
            ("end", _) | (_, "start") => graph.entry(b).or_insert_with(Vec::new).push(a),
            (_, "end") | ("start", _) => graph.entry(a).or_insert_with(Vec::new).push(b),
            _ => {
                graph.entry(a).or_insert_with(Vec::new).push(b);
                graph.entry(b).or_insert_with(Vec::new).push(a);
            }
        }
        graph.insert("end", vec![]);
    }
    graph
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> u32 {
    search("start", graph, &mut HashSet::new())
}

fn part2(graph: &HashMap<&str, Vec<&str>>) -> u32 {
    search2("start", graph, &mut HashSet::new(), &mut None)
}

fn main() {
    let input: String = fs::read_to_string("input/day12").expect("Failed to read input");

    let graph = parse_input(&input);

    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
    const INPUT2: &str = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
    const INPUT3: &str = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT1)), 10);
        assert_eq!(part1(&parse_input(INPUT2)), 19);
        assert_eq!(part1(&parse_input(INPUT3)), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT1)), 36);
        assert_eq!(part2(&parse_input(INPUT2)), 103);
        assert_eq!(part2(&parse_input(INPUT3)), 3509);
    }
}
