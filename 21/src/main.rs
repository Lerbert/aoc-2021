use std::cmp::min;

struct DeterministicDie {
    gen: Box<dyn Iterator<Item=u32>>,
    rolls: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie { gen: Box::new((0..).map(|i| i % 100 + 1)), rolls: 0 }
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

fn main() {
    let (p1, p2, rolls) = practice_game(4, 9);
    println!("{} {} {}", p1, p2, rolls);
    let answer = min(p1, p2) * rolls;
    println!("{}", answer)
}
