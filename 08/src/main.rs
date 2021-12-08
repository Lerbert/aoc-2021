use std::collections::{HashMap, HashSet};
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

impl Segment {
    fn from_char(c: char) -> Result<Self, anyhow::Error> {
        match c {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            x => anyhow::bail!("unexpected token {}", x),
        }
    }
}

#[derive(Clone, Debug)]
struct SevenSegment {
    active_segments: HashSet<Segment>,
}

impl SevenSegment {
    fn get_active_cnt(&self) -> u32 {
        self.active_segments.len() as u32
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
        const EMPTY_VEC: Vec<&HashSet<Segment>> = Vec::new();
        let mut digit_mapping: HashMap<u32, Vec<&HashSet<Segment>>> = [EMPTY_VEC; 10]
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (i as u32, x))
            .collect();
        self.observed_digits.iter().for_each(|d| {
            let insert_positions = match d.get_active_cnt() {
                2 => vec![1],
                3 => vec![7],
                4 => vec![4],
                5 => vec![2, 3, 5],
                6 => vec![0, 6, 9],
                7 => vec![8],
                _ => vec![],
            };
            for pos in insert_positions {
                digit_mapping
                    .get_mut(&pos)
                    .map(|v| v.push(&d.active_segments))
                    .expect("missing key");
            }
        });

        // 1, 4, 7, and 8 are unique, filter by containing 1
        let digit_mapping: HashMap<u32, Vec<&HashSet<Segment>>> = digit_mapping
            .iter()
            .map(|(&k, v)| {
                (
                    k,
                    v.iter()
                        .filter(|s| match k {
                            0 | 3 | 9 => {
                                digit_mapping.get(&1).expect("missing key")[0].is_subset(s)
                            }
                            2 | 5 | 6 => {
                                !digit_mapping.get(&1).expect("missing key")[0].is_subset(s)
                            }
                            _ => true,
                        })
                        .map(|&s| s)
                        .collect(),
                )
            })
            .collect();
        // 1, 3, 4, 6, 7 and 8 are unique
        let digit_mapping: HashMap<u32, Vec<&HashSet<Segment>>> = digit_mapping
            .iter()
            .map(|(&k, v)| {
                (
                    k,
                    v.iter()
                        .filter(|s| match k {
                            // distinguish 0 and 9 by 3 being subset of 9, but not of 0
                            0 => !digit_mapping.get(&3).expect("missing key")[0].is_subset(s),
                            9 => digit_mapping.get(&3).expect("missing key")[0].is_subset(s),
                            // distinguish 2 and 5 by 6 being superset of 5, but not of 2
                            2 => !digit_mapping.get(&6).expect("missing key")[0].is_superset(s),
                            5 => digit_mapping.get(&6).expect("missing key")[0].is_superset(s),
                            _ => true,
                        })
                        .map(|&s| s)
                        .collect(),
                )
            })
            .collect();

        // every digit should be unique now so flatten Vecs
        let digit_mapping: HashMap<u32, &HashSet<Segment>> = digit_mapping
            .iter()
            .map(|(&k, v)| {
                assert_eq!(v.len(), 1);
                (k, v[0])
            })
            .collect();

        self.output_values
            .iter()
            .map(|s| {
                digit_mapping
                    .iter()
                    .find_map(|(&k, &v)| {
                        if *v == s.active_segments {
                            Some(k)
                        } else {
                            None
                        }
                    })
                    .expect("invalid pattern")
            })
            .fold(0, |acc, d| acc * 10 + d)
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

fn main() {
    if let Ok(inputs) = input_parser::parse_inputs::<Display>("./input") {
        let unique_segment_numbers: u32 = inputs
            .iter()
            .map(|d| d.get_unique_segment_number_cnt())
            .sum();
        println!(
            "There are {} digits with unique segment count (i.e. 1, 4, 7, 8)",
            unique_segment_numbers
        );
        let output_sum: u32 = inputs.iter().map(|d| d.determine_output()).sum();
        println!("The sum of all outputs is {}", output_sum);
    }
}
