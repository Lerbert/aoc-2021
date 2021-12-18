use std::fmt::{self, Display, Formatter};
use std::ops::Add;

use itertools::Itertools;
use serde_json::{self, Value};

enum ExplodeResult {
    Left(u32),
    Right(u32),
    LeftRight(u32, u32),
    None,
    Done,
}

enum ExplodeResultApp {
    Left(u32),
    Right(u32),
}

enum SplitResult {
    None,
    Done,
}

#[derive(Clone, Debug)]
pub enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Number(u32),
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SnailfishNumber::Number(n) => write!(f, "{}", n),
            SnailfishNumber::Pair(fst, snd) => write!(f, "[{}, {}]", *fst, *snd),
        }
    }
}

impl From<Value> for SnailfishNumber {
    fn from(j: Value) -> Self {
        match j {
            Value::Array(mut v) => {
                if v.len() != 2 {
                    panic!("pair does not have 2 entries")
                }
                SnailfishNumber::Pair(
                    Box::new(SnailfishNumber::from(v.remove(0))),
                    Box::new(SnailfishNumber::from(v.remove(0))),
                )
            }
            Value::Number(n) => {
                SnailfishNumber::Number(n.as_u64().expect("malformed number") as u32)
            }
            _ => panic!("unexpected element in snailfish number"),
        }
    }
}

impl SnailfishNumber {
    pub fn reduce(self) -> SnailfishNumber {
        let x = self;
        let (x, res_e) = x.explode_helper(0);
        if let ExplodeResult::None = res_e {
            let (x, res_s) = x.split_helper();
            if let SplitResult::None = res_s {
                x
            } else {
                x.reduce()
            }
        } else {
            x.reduce()
        }
    }

    pub fn explode(self) -> SnailfishNumber {
        self.explode_helper(0).0
    }

    pub fn split(self) -> SnailfishNumber {
        self.split_helper().0
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            SnailfishNumber::Number(n) => *n,
            SnailfishNumber::Pair(fst, snd) => 3 * fst.magnitude() + 2 * snd.magnitude(),
        }
    }

    fn explode_helper(self, nesting_level: u8) -> (SnailfishNumber, ExplodeResult) {
        match self {
            SnailfishNumber::Pair(fst, snd) => {
                if nesting_level >= 4 {
                    let (fst, snd) = (*fst, *snd);
                    match (fst, snd) {
                        (SnailfishNumber::Number(l), SnailfishNumber::Number(r)) => {
                            (SnailfishNumber::Number(0), ExplodeResult::LeftRight(l, r))
                        }
                        (fst, snd) => {
                            Self::explode_pair(Box::new(fst), Box::new(snd), nesting_level)
                        }
                    }
                } else {
                    Self::explode_pair(fst, snd, nesting_level)
                }
            }
            SnailfishNumber::Number(_) => (self, ExplodeResult::None),
        }
    }

    fn explode_pair(
        fst: Box<SnailfishNumber>,
        snd: Box<SnailfishNumber>,
        nesting_level: u8,
    ) -> (SnailfishNumber, ExplodeResult) {
        let (fst, res) = fst.explode_helper(nesting_level + 1);
        let fst = Box::new(fst);
        match res {
            ExplodeResult::Left(l) => (SnailfishNumber::Pair(fst, snd), ExplodeResult::Left(l)),
            ExplodeResult::Right(r) => (
                SnailfishNumber::Pair(fst, snd.apply_explode_result(ExplodeResultApp::Left(r))),
                ExplodeResult::Done,
            ),
            ExplodeResult::LeftRight(l, r) => (
                SnailfishNumber::Pair(fst, snd.apply_explode_result(ExplodeResultApp::Left(r))),
                ExplodeResult::Left(l),
            ),
            ExplodeResult::Done => (SnailfishNumber::Pair(fst, snd), ExplodeResult::Done),
            ExplodeResult::None => {
                let (snd, res) = snd.explode_helper(nesting_level + 1);
                let snd = Box::new(snd);
                match res {
                    ExplodeResult::Left(l) => (
                        SnailfishNumber::Pair(
                            fst.apply_explode_result(ExplodeResultApp::Right(l)),
                            snd,
                        ),
                        ExplodeResult::Done,
                    ),
                    ExplodeResult::Right(r) => {
                        (SnailfishNumber::Pair(fst, snd), ExplodeResult::Right(r))
                    }
                    ExplodeResult::LeftRight(l, r) => (
                        SnailfishNumber::Pair(
                            fst.apply_explode_result(ExplodeResultApp::Right(l)),
                            snd,
                        ),
                        ExplodeResult::Right(r),
                    ),
                    ExplodeResult::Done => (SnailfishNumber::Pair(fst, snd), ExplodeResult::Done),
                    ExplodeResult::None => (SnailfishNumber::Pair(fst, snd), ExplodeResult::None),
                }
            }
        }
    }

    fn apply_explode_result(self, result: ExplodeResultApp) -> Box<SnailfishNumber> {
        let sfn = match result {
            ExplodeResultApp::Left(l) => match self {
                SnailfishNumber::Number(n) => SnailfishNumber::Number(n + l),
                SnailfishNumber::Pair(fst, snd) => {
                    SnailfishNumber::Pair(fst.apply_explode_result(ExplodeResultApp::Left(l)), snd)
                }
            },
            ExplodeResultApp::Right(r) => match self {
                SnailfishNumber::Number(n) => SnailfishNumber::Number(n + r),
                SnailfishNumber::Pair(fst, snd) => {
                    SnailfishNumber::Pair(fst, snd.apply_explode_result(ExplodeResultApp::Right(r)))
                }
            },
        };
        Box::new(sfn)
    }

    fn split_helper(self) -> (SnailfishNumber, SplitResult) {
        match self {
            SnailfishNumber::Number(n) if n >= 10 => (
                SnailfishNumber::Pair(
                    Box::new(SnailfishNumber::Number(n / 2)),
                    Box::new(SnailfishNumber::Number(n / 2 + n % 2)),
                ),
                SplitResult::Done,
            ),
            SnailfishNumber::Number(n) if n < 10 => (SnailfishNumber::Number(n), SplitResult::None),
            SnailfishNumber::Pair(fst, snd) => {
                let (fst, res) = fst.split_helper();
                let fst = Box::new(fst);
                match res {
                    SplitResult::Done => (SnailfishNumber::Pair(fst, snd), SplitResult::Done),
                    SplitResult::None => {
                        let (snd, res) = snd.split_helper();
                        let snd = Box::new(snd);
                        (SnailfishNumber::Pair(fst, snd), res)
                    }
                }
            }
            _ => panic!("should not be reached"),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let result = SnailfishNumber::Pair(Box::new(self), Box::new(other));
        result.reduce()
    }
}

fn main() {
    let inputs = include_str!("../input").trim();
    let inputs: Vec<_> = inputs
        .split('\n')
        .map(|s| -> Value { serde_json::from_str(s).expect("malformed number") })
        .map(|v| SnailfishNumber::from(v))
        .collect();
    let result = inputs
        .clone()
        .into_iter()
        .reduce(|acc, v| acc + v)
        .expect("empty input");
    let mag = result.magnitude();
    println!(
        "Adding up all numbers yields {} (Magnitude {})",
        result, mag
    );

    let largest = inputs
        .clone()
        .into_iter()
        .permutations(2)
        .map(|mut v| v.remove(0) + v.remove(0))
        .map(|n| n.magnitude())
        .max()
        .expect("empty input");
    println!(
        "The largest magnitude we can obtain by adding two numbers from the list is {}",
        largest
    )
}
