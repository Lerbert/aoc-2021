use std::collections::HashSet;
use std::str::FromStr;

use anyhow;

use input_parser;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
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
            .map(|c| match c {
                'a' => Ok(Segment::A),
                'b' => Ok(Segment::B),
                'c' => Ok(Segment::C),
                'd' => Ok(Segment::D),
                'e' => Ok(Segment::E),
                'f' => Ok(Segment::F),
                'g' => Ok(Segment::G),
                x => anyhow::bail!("unexpected token {}", x),
            })
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
        let unique_segment_numbers: u32 = inputs.iter().map(|d| d.get_unique_segment_number_cnt()).sum();
        println!("There are {} digits with unique segment count (i.e. 1, 4, 7, 8)", unique_segment_numbers)
    }
}
