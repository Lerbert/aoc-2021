use std::cmp::{max, min};
use std::collections::HashMap;

use itertools::iproduct;

struct DeterministicDie {
    gen: Box<dyn Iterator<Item = u32>>,
    rolls: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie {
            gen: Box::new((0..).map(|i| i % 100 + 1)),
            rolls: 0,
        }
    }

    fn next(&mut self) -> u32 {
        self.rolls += 1;
        self.gen.next().expect("die has run out of values")
    }
}

fn roll(die: &mut DeterministicDie) -> u32 {
    die.next() + die.next() + die.next()
}

fn practice_game(p1_start: u32, p2_start: u32) -> (u32, u32, u32) {
    let mut die = DeterministicDie::new();
    let mut p1 = (p1_start - 1, 0);
    let mut p2 = (p2_start - 1, 0);
    let mut p1_turn = true;
    while p1.1 < 1000 && p2.1 < 1000 {
        let roll = roll(&mut die);
        if p1_turn {
            p1.0 += roll;
            p1.0 %= 10;
            p1.1 += p1.0 + 1
        } else {
            p2.0 += roll;
            p2.0 %= 10;
            p2.1 += p2.0 + 1
        }
        p1_turn = !p1_turn;
    }
    (p1.1, p2.1, die.rolls)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: u32,
    score: u32,
}

struct DiracPredictor {
    mins: HashMap<State, u32>,
    maxs: HashMap<State, u32>,
    win_cache: HashMap<(State, State), (u128, u128)>,
    win_from_cache: HashMap<State, u128>,
}

impl DiracPredictor {
    fn new() -> Self {
        DiracPredictor {
            mins: HashMap::new(),
            maxs: HashMap::new(),
            win_cache: HashMap::new(),
            win_from_cache: HashMap::new(),
        }
    }

    fn min_turns_to_win(&mut self, state: &State) -> u32 {
        if let Some(&steps) = self.mins.get(state) {
            steps
        } else if state.score >= 21 {
            0
        } else {
            let steps = Self::turn(state)
                .iter()
                .map(|s| self.min_turns_to_win(&s))
                .min()
                .unwrap()
                + 1;
            self.mins.insert(*state, steps);
            steps
        }
    }

    fn max_turns_to_win(&mut self, state: &State) -> u32 {
        if let Some(&steps) = self.maxs.get(state) {
            steps
        } else if state.score >= 21 {
            0
        } else {
            let steps = Self::turn(state)
                .iter()
                .map(|s| self.max_turns_to_win(&s))
                .max()
                .unwrap()
                + 1;
            self.maxs.insert(*state, steps);
            steps
        }
    }

    fn turn(state: &State) -> Vec<State> {
        iproduct!(1..=3, 1..=3, 1..=3)
            .map(|(i, j, k)| i + j + k)
            .map(|roll| {
                let pos = (state.pos - 1 + roll) % 10 + 1;
                let score = state.score + pos;
                State { pos, score }
            })
            .collect()
    }

    fn wins(&mut self, p1: &State, p2: &State) -> (u128, u128) {
        if let Some(&w) = self.win_cache.get(&(*p1, *p2)) {
            return w;
        }

        let p1_max = self.max_turns_to_win(p1);
        let p2_min = self.min_turns_to_win(p2);
        let res = if p1_max <= p2_min {
            // P1 wins, since they go first
            (self.wins_from(p1), 0)
        } else {
            // Winner unknown --> let the current player P1 take a turn and flip P1 and P2 for the next step
            DiracPredictor::turn(p1)
                .iter()
                .map(|p1_next| {
                    let max_turns = self.max_turns_to_win(p1_next);
                    let min_turns = self.min_turns_to_win(p1_next);
                    if min_turns == max_turns && min_turns == 0 {
                        // P1 wins now, before P2 gets to create universes
                        (0, 1)
                    } else {
                        self.wins(p2, &p1_next)
                    }
                })
                .fold((0, 0), |acc, w| (acc.0 + w.1, acc.1 + w.0))
        };
        self.win_cache.insert((*p1, *p2), res);
        res
    }

    fn wins_from(&mut self, state: &State) -> u128 {
        if let Some(&w) = self.win_from_cache.get(&state) {
            return w;
        }

        let max_turns = self.max_turns_to_win(state);
        let min_turns = self.min_turns_to_win(state);
        let res = if min_turns == max_turns {
            if min_turns == 0 {
                1
            } else {
                let turns = 2 * min_turns - 1;
                27_u128.pow(turns)
            }
        } else {
            DiracPredictor::turn(state)
                .iter()
                .map(|s| {
                    let max_turns = self.max_turns_to_win(s);
                    let min_turns = self.min_turns_to_win(s);
                    if min_turns == max_turns && min_turns == 0 {
                        // P1 wins now, before P2 gets to create universes
                        1
                    } else {
                        27 * self.wins_from(&s)
                    }
                })
                .sum::<u128>()
        };
        self.win_from_cache.insert(*state, res);
        res
    }
}

fn main() {
    let (p1, p2, rolls) = practice_game(4, 9);
    let answer = min(p1, p2) * rolls;
    println!(
        "Player 1 {} : {} Player 2 after {} turns (solution: {})",
        p1, p2, rolls, answer
    );

    let p1 = State { pos: 4, score: 0 };
    let p2 = State { pos: 9, score: 0 };
    let mut pred = DiracPredictor::new();
    let wins = pred.wins(&p1, &p2);
    println!(
        "Player 1 wins in {} universes, Player 2 in {} (solution: {})",
        wins.0,
        wins.1,
        max(wins.0, wins.1)
    );
}
