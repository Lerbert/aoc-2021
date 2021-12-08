use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;
use std::process::Command;
use std::str::FromStr;

use anyhow;

use input_parser;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

static ZERO: [Segment; 6] = [
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::E,
    Segment::F,
    Segment::G,
];
static ONE: [Segment; 2] = [Segment::C, Segment::F];
static TWO: [Segment; 5] = [Segment::A, Segment::C, Segment::D, Segment::E, Segment::G];
static THREE: [Segment; 5] = [Segment::A, Segment::C, Segment::D, Segment::F, Segment::G];
static FOUR: [Segment; 4] = [Segment::B, Segment::C, Segment::D, Segment::F];
static FIVE: [Segment; 5] = [Segment::A, Segment::B, Segment::D, Segment::F, Segment::G];
static SIX: [Segment; 6] = [
    Segment::A,
    Segment::B,
    Segment::D,
    Segment::E,
    Segment::F,
    Segment::G,
];
static SEVEN: [Segment; 3] = [Segment::A, Segment::C, Segment::F];
static EIGHT: [Segment; 7] = [
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::E,
    Segment::F,
    Segment::G,
];
static NINE: [Segment; 6] = [
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::F,
    Segment::G,
];

impl Segment {
    fn from_char(c: char) -> Result<Self, anyhow::Error> {
        match c {
            'A' | 'a' | '1' => Ok(Segment::A),
            'B' | 'b' | '2' => Ok(Segment::B),
            'C' | 'c' | '3' => Ok(Segment::C),
            'D' | 'd' | '4' => Ok(Segment::D),
            'E' | 'e' | '5' => Ok(Segment::E),
            'F' | 'f' | '6' => Ok(Segment::F),
            'G' | 'g' | '7' => Ok(Segment::G),
            x => anyhow::bail!("unexpected token {}", x),
        }
    }
}

#[derive(Debug)]
struct SevenSegment {
    active_segments: HashSet<Segment>,
}

impl SevenSegment {
    fn get_active_cnt(&self) -> u32 {
        self.active_segments.len() as u32
    }

    fn get_digit(&self, translation: &HashMap<Segment, Segment>) -> Option<u32> {
        let real_active: HashSet<Segment> = self
            .active_segments
            .iter()
            .map(|s| *translation.get(s).expect("missing translation"))
            .collect();
            println!("{:?}", real_active);
        match real_active {
            x if x == ZERO.iter().cloned().collect() => Some(0),
            x if x == ONE.iter().cloned().collect() => Some(1),
            x if x == TWO.iter().cloned().collect() => Some(2),
            x if x == THREE.iter().cloned().collect() => Some(3),
            x if x == FOUR.iter().cloned().collect() => Some(4),
            x if x == FIVE.iter().cloned().collect() => Some(5),
            x if x == SIX.iter().cloned().collect() => Some(6),
            x if x == SEVEN.iter().cloned().collect() => Some(7),
            x if x == EIGHT.iter().cloned().collect() => Some(8),
            x if x == NINE.iter().cloned().collect() => Some(9),
            _ => None,
        }
    }
}

impl FromStr for SevenSegment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let active_segments = s
            .chars()
            .map(|c| Segment::from_char(c))
            .collect::<Result<HashSet<Segment>, Self::Err>>()?;
        Ok(SevenSegment { active_segments })
    }
}

#[derive(Debug)]
struct Display {
    observed_digits: Vec<SevenSegment>,
    output_values: Vec<SevenSegment>,
}

impl Display {
    fn get_unique_segment_number_cnt(&self) -> u32 {
        self.output_values
            .iter()
            .filter_map(|digit| match digit.get_active_cnt() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            })
            .count() as u32
    }

    fn determine_output(&self) -> u32 {
        let mut segment_mapping: HashMap<Segment, HashSet<Segment>> = HashMap::new();
        self.observed_digits.iter().for_each(|d| {
            let possible_interpretations = HashSet::from_iter(match d.get_active_cnt() {
                2 => vec![Segment::C, Segment::F],                         // 1
                3 => vec![Segment::A, Segment::C, Segment::F],             // 7
                4 => vec![Segment::B, Segment::C, Segment::D, Segment::F], // 4
                5 => vec![
                    Segment::A,
                    Segment::B,
                    Segment::C,
                    Segment::D,
                    Segment::E,
                    Segment::F,
                    Segment::G,
                ], // 2, 3, 5
                6 => vec![
                    Segment::A,
                    Segment::B,
                    Segment::C,
                    Segment::D,
                    Segment::E,
                    Segment::F,
                    Segment::G,
                ], // 0, 6, 9
                7 => vec![
                    Segment::A,
                    Segment::B,
                    Segment::C,
                    Segment::D,
                    Segment::E,
                    Segment::F,
                    Segment::G,
                ], // 8
                _ => vec![],
            });
            for pos in &d.active_segments {
                let possible_digits = segment_mapping
                    .get(&pos)
                    .map(|v| {
                        v.intersection(&possible_interpretations)
                            .map(|&s| s)
                            .collect()
                    })
                    .unwrap_or(possible_interpretations.clone());
                segment_mapping.insert(*pos, possible_digits);
            }
        });
        let translation = solve_minizinc(&segment_mapping);
        println!("{:?}", translation);
        self.output_values
            .iter()
            .map(|s| s.get_digit(&translation).expect("malformed seven segment"))
            .fold(0, |acc, d| (acc * 10) + d)
    }
}

impl FromStr for Display {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('|').map(|s| s.trim()).collect();
        if split.len() != 2 {
            anyhow::bail!("Malformed string {}", s);
        }
        let observed_digits = split[0]
            .split(' ')
            .map(|s| s.parse::<SevenSegment>())
            .collect::<Result<Vec<SevenSegment>, Self::Err>>()?;
        let output_values = split[1]
            .split(' ')
            .map(|s| s.parse::<SevenSegment>())
            .collect::<Result<Vec<SevenSegment>, Self::Err>>()?;
        Ok(Display {
            observed_digits,
            output_values,
        })
    }
}

fn solve_minizinc(constraints: &HashMap<Segment, HashSet<Segment>>) -> HashMap<Segment, Segment> {
    let constraints_mzn: String = constraints
        .iter()
        .map(|(seg, possible)| format!("constraint {:?}_ in {:?};", seg, possible))
        .collect();
    let model = format!(
        "include \"alldifferent.mzn\";

    enum Segment = {{A, B, C, D, E, F, G}};
    
    var int: A_;
    var int: B_;
    var int: C_;
    var int: D_;
    var int: E_;
    var int: F_;
    var int: G_;
    
    constraint alldifferent([A_, B_, C_, D_, E_, F_, G_]);
    
    {}
    
    solve satisfy;",
        constraints_mzn
    );
    fs::write("./tmp.mzn", model).expect("unable to write model");
    let solution = String::from_utf8(
        Command::new("mzn-fzn")
            .arg("./tmp.mzn")
            .output()
            .expect("MiniZinc failed")
            .stdout,
    )
    .expect("no UTF8 output");

    solution
        .split('\n')
        .filter(|&s| s != "")
        .map(|s| {
            (
                s.chars().nth(0).expect("empty line"),
                s.chars().rev().nth(1).expect("empty line"),
            )
        })
        .filter_map(|(c1, c2)| {
            if let (Ok(seg1), Ok(seg2)) = (Segment::from_char(c1), Segment::from_char(c2)) {
                Some((seg1, seg2))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    if let Ok(inputs) = input_parser::parse_inputs::<Display>("./test") {
        let unique_segment_numbers: u32 = inputs
            .iter()
            .map(|d| d.get_unique_segment_number_cnt())
            .sum();
        println!(
            "There are {} digits with unique segment count (i.e. 1, 4, 7, 8)",
            unique_segment_numbers
        );
        println!("{}", inputs[0].determine_output());
    }
}
