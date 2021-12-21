use std::collections::HashMap;

type ParsedInput = (usize, usize);
type PlayerStates = HashMap<Player, usize>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Player { score: 0, position }
    }

    fn step(&mut self, n: usize) {
        self.position = (((self.position - 1) + n) % 10) + 1;
        self.score += self.position;
    }

    fn is_win(&self, n: usize) -> bool {
        self.score >= n
    }

    fn player_wins(&self, n: usize) -> Vec<(usize, usize)> {
        let roll_count: HashMap<usize, usize> =
            HashMap::from([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);

        let mut player_turns = vec![(1, 0)];
        let mut player_states: PlayerStates = HashMap::new();
        player_states.insert(*self, 1);

        for turn in 1..12 {
            player_turns.push((0, 0));

            let mut next_player_states: PlayerStates = HashMap::new();

            for (&player, &count) in &player_states {
                for (&r, &r_count) in &roll_count {
                    let mut p = player;
                    p.step(r);
                    if p.is_win(n) {
                        player_turns[turn].1 += count * r_count;
                    } else {
                        player_turns[turn].0 += count * r_count;
                        *next_player_states.entry(p).or_insert(0) += count * r_count;
                    }
                }
            }
            if player_turns[turn] == (0, 0) {
                player_turns.pop();
                break;
            }
            player_states = next_player_states;
        }
        player_turns
    }
}

fn part1(input: ParsedInput) -> usize {
    let (n, m) = input;

    let mut player1 = Player::new(n);
    let mut player2 = Player::new(m);

    let mut i = 0;
    loop {
        player1.step(3 * i + 6);
        i += 3;
        if player1.is_win(1000) {
            break;
        }

        player2.step(3 * i + 6);
        i += 3;
        if player2.is_win(1000) {
            break;
        }
    }

    player1.score.min(player2.score) * i
}

fn part2(input: ParsedInput) -> usize {
    let player1 = Player::new(input.0);
    let player2 = Player::new(input.1);

    let player1_wins = player1.player_wins(21);
    let player2_wins = player2.player_wins(21);

    let mut wins = [0, 0];

    for i in 0..player1_wins.len() {
        let (loss, _) = player1_wins[i];
        let (_, win) = player2_wins[i];
        wins[1] += win * loss;
    }

    for i in 1..player1_wins.len() {
        let (_, win) = player1_wins[i];
        let (loss, _) = player2_wins[i - 1];
        wins[0] += win * loss;
    }

    *wins.iter().max().unwrap()
}

fn main() {
    let input = (9, 10);

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: (usize, usize) = (4, 8);

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 739_785);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 444_356_092_776_315);
    }
}
