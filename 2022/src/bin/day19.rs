use nom::bytes::complete::tag;
use nom::IResult;
use std::fs;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

impl Blueprint {
    fn get_geodes(&self, minutes: u32) -> u32 {
        let mut max_state = State::new();
        let mut s = vec![State::new()];

        while let Some(v) = s.pop() {
            let n = minutes - v.t;
            if n * (n + 1) / 3 + n * v.geode_robot + v.geode <= max_state.geode {
            } else if v.t < minutes {
                for w in v.next(self) {
                    s.push(w);
                }
            } else if v.geode > max_state.geode {
                max_state = v;
            }
        }
        max_state.geode
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    t: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
}

impl State {
    fn new() -> Self {
        Self {
            t: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }

    fn next(&self, blueprint: &Blueprint) -> Vec<State> {
        let stepped_state = State {
            t: self.t + 1,
            ore: self.ore + self.ore_robot,
            clay: self.clay + self.clay_robot,
            obsidian: self.obsidian + self.obsidian_robot,
            geode: self.geode + self.geode_robot,
            ..*self
        };

        if blueprint.geode.0 <= self.ore && blueprint.geode.1 <= self.obsidian {
            return vec![State {
                ore: stepped_state.ore - blueprint.geode.0,
                obsidian: stepped_state.obsidian - blueprint.geode.1,
                geode_robot: stepped_state.geode_robot + 1,
                ..stepped_state
            }];
        }

        let mut states = vec![];
        if blueprint.obsidian.0 <= self.ore && blueprint.obsidian.1 <= self.clay {
            states.push(State {
                ore: stepped_state.ore - blueprint.obsidian.0,
                clay: stepped_state.clay - blueprint.obsidian.1,
                obsidian_robot: stepped_state.obsidian_robot + 1,
                ..stepped_state
            });
        }
        if blueprint.clay <= self.ore {
            states.push(State {
                ore: stepped_state.ore - blueprint.clay,
                clay_robot: stepped_state.clay_robot + 1,
                ..stepped_state
            });
        }
        if blueprint.ore <= self.ore {
            states.push(State {
                ore: stepped_state.ore - blueprint.ore,
                ore_robot: stepped_state.ore_robot + 1,
                ..stepped_state
            });
        }
        if self.ore < blueprint.ore
            || self.ore < blueprint.clay
            || self.ore < 2 * blueprint.obsidian.0
            || self.ore < 2 * blueprint.geode.0
        {
            states.push(State { ..stepped_state });
        }

        states
    }
}

fn part1(blueprints: &[Blueprint]) -> u32 {
    let mut ans = 0;
    for blueprint in blueprints {
        ans += blueprint.get_geodes(24) * blueprint.id;
    }
    ans
}

fn part2(blueprints: &[Blueprint]) -> u32 {
    let mut ans = 1;
    for blueprint in blueprints.iter().take(3) {
        ans *= blueprint.get_geodes(32);
    }
    ans
}

fn parse(input: &str) -> Vec<Blueprint> {
    let result: IResult<_, _> = nom::multi::separated_list0(
        tag("\n"),
        nom::combinator::map(
            nom::sequence::tuple((
                tag("Blueprint "),
                nom::character::complete::u32,
                tag(": Each ore robot costs "),
                nom::character::complete::u32,
                tag(" ore. Each clay robot costs "),
                nom::character::complete::u32,
                tag(" ore. Each obsidian robot costs "),
                nom::character::complete::u32,
                tag(" ore and "),
                nom::character::complete::u32,
                tag(" clay. Each geode robot costs "),
                nom::character::complete::u32,
                tag(" ore and "),
                nom::character::complete::u32,
                tag(" obsidian."),
            )),
            |(_, id, _, ore, _, clay, _, obsidian1, _, obsidian2, _, geode1, _, geode2, _)| {
                Blueprint {
                    id,
                    ore,
                    clay,
                    obsidian: (obsidian1, obsidian2),
                    geode: (geode1, geode2),
                }
            },
        ),
    )(input);
    result.unwrap().1
}

fn main() {
    let input: String = fs::read_to_string("input/day19").expect("Failed to read input");
    let blueprints = parse(&input);

    println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 33);
    }

    #[test]
    fn test_part2() {
        let blueprints = parse(TEST_INPUT);
        assert_eq!(blueprints[0].get_geodes(32), 56);
        assert_eq!(blueprints[1].get_geodes(32), 62);
    }
}
