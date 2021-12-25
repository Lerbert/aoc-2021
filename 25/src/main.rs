use std::fmt::{self, Display};

use anyhow::Result;
use peg;

peg::parser! {
    grammar map_parser() for str {
        rule field() -> Option<SeaCucumber>
            = "v" { Some(SeaCucumber::South) } / ">" { Some(SeaCucumber::East) } / "." { None }

        rule row() -> Vec<Option<SeaCucumber>>
            = field()*

        pub rule map() -> Map
            = rows:row() ** "\n" {?
                if rows.is_empty() {
                    Ok(Map { data: Vec::new(), height: 0, width: 0 })
                } else {
                    let width = rows[0].len();
                    if rows.iter().any(|v| v.len() != width) {
                        Err("different row lengths")
                    } else {
                        let height = rows.len();
                        let data = rows.into_iter().flatten().collect();
                        Ok(Map { data, height, width })
                    }
                }
            }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SeaCucumber {
    East,
    South,
}

impl Display for SeaCucumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::East => write!(f, ">"),
            Self::South => write!(f, "v"),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    data: Vec<Option<SeaCucumber>>,
    height: usize,
    width: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, v) in self.data.iter().enumerate() {
            if idx % self.width == 0 {
                write!(f, "\n")?;
            }
            match v {
                None => write!(f, ".")?,
                Some(c) => write!(f, "{}", c)?,
            }
        }
        Ok(())
    }
}

impl Map {
    pub fn get(&self, i: isize, j: isize) -> Option<SeaCucumber> {
        let i = Self::calculate_idx(i, self.height);
        let j = Self::calculate_idx(j, self.width);
        self.data[i * self.width + j]
    }

    pub fn set(&mut self, i: isize, j: isize, v: Option<SeaCucumber>) {
        let i = Self::calculate_idx(i, self.height);
        let j = Self::calculate_idx(j, self.width);
        self.data[i * self.width + j] = v
    }

    pub fn iter_row(&self, i: isize) -> impl Iterator<Item = &Option<SeaCucumber>> {
        let i = Self::calculate_idx(i, self.height);
        self.data
            .iter()
            .enumerate()
            .filter(move |(idx, _)| *idx >= i * self.width && *idx < (i + 1) * self.width)
            .map(|t| t.1)
    }

    pub fn iter_col(&self, j: isize) -> impl Iterator<Item = &Option<SeaCucumber>> {
        let j = Self::calculate_idx(j, self.width);
        self.data
            .iter()
            .enumerate()
            .filter(move |(idx, _)| j == idx % self.width)
            .map(|t| t.1)
    }

    fn calculate_idx(idx: isize, max: usize) -> usize {
        let idx = idx % max as isize;
        if idx < 0 {
            max - idx.abs() as usize
        } else {
            idx as usize
        }
    }
}

#[derive(Debug)]
pub struct Simulation {
    map: Map,
}

impl Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

impl Simulation {
    pub fn simulate_until_no_moves(&mut self) -> Option<u32> {
        for i in 1.. {
            if !self.step() {
                println!("\n{}{}", i, self);
                return Some(i);
            }
            if i % 10 == 0 {
                println!("\n{}{}", i, self);
            }
        }
        None
    }
    pub fn step(&mut self) -> bool {
        let mut changed = false;
        changed |= self.move_east();
        changed |= self.move_south();
        changed
    }

    fn move_east(&mut self) -> bool {
        let mut changed = false;
        for i in 0..self.map.height {
            let does_move: Vec<_> = self
                .map
                .iter_row(i as isize)
                .zip(
                    self.map
                        .iter_row(i as isize)
                        .skip(1)
                        .chain(self.map.iter_row(i as isize)),
                )
                .map(|(c, n)| match n {
                    Some(_) => false,
                    None => match c {
                        Some(SeaCucumber::East) => true,
                        _ => false,
                    },
                })
                .collect();
            for (j, does_move) in does_move.iter().enumerate() {
                if *does_move {
                    self.map.set(i as isize, j as isize, None);
                    self.map
                        .set(i as isize, j as isize + 1, Some(SeaCucumber::East));
                }
            }
            changed |= does_move.iter().any(|&b| b);
        }
        changed
    }

    fn move_south(&mut self) -> bool {
        let mut changed = false;
        for j in 0..self.map.width {
            let does_move: Vec<_> = self
                .map
                .iter_col(j as isize)
                .zip(
                    self.map
                        .iter_col(j as isize)
                        .skip(1)
                        .chain(self.map.iter_col(j as isize)),
                )
                .map(|(c, n)| match n {
                    Some(_) => false,
                    None => match c {
                        Some(SeaCucumber::South) => true,
                        _ => false,
                    },
                })
                .collect();
            for (i, does_move) in does_move.iter().enumerate() {
                if *does_move {
                    self.map.set(i as isize, j as isize, None);
                    self.map
                        .set(i as isize + 1, j as isize, Some(SeaCucumber::South));
                }
            }
            changed |= does_move.iter().any(|&b| b);
        }
        changed
    }
}

fn main() -> Result<()> {
    let inputs = include_str!("../input").trim();
    let mut sim = Simulation {
        map: map_parser::map(inputs)?,
    };
    let stops_after = sim
        .simulate_until_no_moves()
        .expect("simulation never stops");
    println!(
        "After {} steps the sea cucumbers don't move anymore",
        stops_after
    );

    Ok(())
}
