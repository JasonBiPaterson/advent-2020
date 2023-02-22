use nom::{bytes::complete::tag, IResult, Parser};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
struct Valve<'a> {
    label: &'a str,
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

fn parse(input: &str) -> Vec<Valve> {
    let a: IResult<_, _> = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::sequence::tuple((
            tag("Valve "),
            nom::bytes::complete::take(2usize),
            tag(" has flow rate="),
            nom::character::complete::u32,
            nom::branch::alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            nom::multi::separated_list1(tag(", "), nom::bytes::complete::take(2usize)),
        ))
        .map(|(_, label, _, flow_rate, _, tunnels)| Valve {
            label,
            flow_rate,
            tunnels,
        }),
    )(input);
    a.unwrap().1
}

fn find_path(
    node: &str,
    time_remaining: u32,
    flow: u32,
    remaining_labels: &HashSet<&str>,
    valve_lookup: &HashMap<&str, &Valve>,
    steps_lookup: &HashMap<(&str, &str), u32>,
) -> u32 {
    let mut max_flow = flow;
    for &label in remaining_labels {
        let steps = steps_lookup.get(&(node, label)).unwrap() + 1;
        if steps <= time_remaining {
            let mut remaining_labels = remaining_labels.clone();
            remaining_labels.remove(label);

            let next_time = time_remaining - steps;

            max_flow = max_flow.max(find_path(
                label,
                next_time,
                flow + next_time * valve_lookup.get(label).unwrap().flow_rate,
                &remaining_labels,
                valve_lookup,
                steps_lookup,
            ));
        }
    }
    max_flow
}

fn calculate_flow(
    labels: &[&str],
    mut remaining_time: u32,
    valve_lookup: &HashMap<&str, &Valve>,
    steps_lookup: &HashMap<(&str, &str), u32>,
) -> u32 {
    let mut total_flow = 0;
    for window in labels.windows(2) {
        let a = window[0];
        let b = window[1];
        remaining_time -= steps_lookup.get(&(a, b)).unwrap() + 1;
        total_flow += remaining_time * valve_lookup.get(b).unwrap().flow_rate;
    }
    total_flow
}

fn gen_paths<'a>(
    paths: &mut HashSet<Vec<&'a str>>,
    // node: &str,
    path: &[&'a str],
    time_remaining: u32,
    remaining_labels: &HashSet<&'a str>,
    steps_lookup: &HashMap<(&str, &str), u32>,
) {
    if remaining_labels.is_empty() {
        paths.insert(path.to_vec());
    }
    for &label in remaining_labels {
        let steps = steps_lookup.get(&(path.last().unwrap(), label)).unwrap() + 1;
        if steps <= time_remaining {
            let mut remaining_labels = remaining_labels.clone();
            remaining_labels.remove(label);

            let next_time = time_remaining - steps;
            let mut next_path = path.to_vec();
            next_path.push(label);

            gen_paths(
                paths,
                &next_path,
                next_time,
                &remaining_labels,
                steps_lookup,
            );
        } else {
            paths.insert(path.to_vec());
        }
    }
}

fn process_valves<'a>(
    valves: &'a [Valve<'a>],
) -> (
    HashSet<&'a str>,
    HashMap<&'a str, &'a Valve<'a>>,
    HashMap<(&'a str, &'a str), u32>,
) {
    let mut valve_lookup: HashMap<&str, &Valve> = HashMap::new();
    for valve in valves {
        valve_lookup.insert(valve.label, valve);
    }

    let nonzero_labels: HashSet<&str> = valves
        .iter()
        .filter(|&valve| valve.flow_rate > 0)
        .map(|valve| valve.label)
        .collect();

    let mut start_labels: HashSet<&str> = nonzero_labels.clone();
    start_labels.insert("AA");
    let mut steps_lookup: HashMap<(&str, &str), u32> = HashMap::new();
    for &start in &start_labels {
        for &end in &nonzero_labels {
            if start == end {
                continue;
            }
            let mut q = VecDeque::new();
            let mut explored = HashSet::new();
            let mut parents = HashMap::new();

            q.push_back(start);
            explored.insert(start);
            while let Some(v) = q.pop_front() {
                if v == end {
                    break;
                }

                for &w in &valve_lookup.get(v).unwrap().tunnels {
                    if !explored.contains(w) {
                        explored.insert(w);
                        parents.insert(w, v);
                        q.push_back(w);
                    }
                }
            }
            let mut a = end;
            let mut steps = 0;
            while let Some(&p) = parents.get(&a) {
                steps += 1;
                a = p;
            }
            steps_lookup.insert((start, end), steps);
        }
    }
    (nonzero_labels, valve_lookup, steps_lookup)
}

fn part1(valves: &[Valve]) -> u32 {
    let (nonzero_labels, valve_lookup, steps_lookup) = process_valves(valves);

    find_path("AA", 30, 0, &nonzero_labels, &valve_lookup, &steps_lookup)
}

fn part2(valves: &[Valve]) -> u32 {
    let (nonzero_labels, valve_lookup, steps_lookup) = process_valves(valves);

    let mut paths = HashSet::new();
    gen_paths(&mut paths, &["AA"], 26, &nonzero_labels, &steps_lookup);

    let mut max_flow = 0;
    for path in paths {
        let mut remaining_labels = nonzero_labels.clone();
        for &label in &path {
            remaining_labels.remove(label);
        }
        let new_flow = calculate_flow(&path, 26, &valve_lookup, &steps_lookup)
            + find_path("AA", 26, 0, &remaining_labels, &valve_lookup, &steps_lookup);
        max_flow = max_flow.max(new_flow);
    }
    max_flow
}

fn main() {
    let input: String = fs::read_to_string("input/day16").expect("Failed to read input");
    let valves = parse(&input);

    println!("Part 1: {}", part1(&valves));
    println!("Part 2: {}", part2(&valves));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 1707);
    }
}
