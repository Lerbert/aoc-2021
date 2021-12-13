use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Error, Debug)]
enum ParsePointError {
    #[error("invalid fold format (expected \"<x>,<y>\", got {0})")]
    UnexpectedFormat(String),
    #[error("error parsing coordinate {0}")]
    ParseCoordinate(#[from] ParseIntError),
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<i32> = s.split(',').map(|s| s.parse()).collect::<Result<_, _>>()?;
        if split.len() != 2 {
            return Err(ParsePointError::UnexpectedFormat(String::from(s)));
        }
        Ok(Point {
            x: split[0],
            y: split[1],
        })
    }
}

enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Error, Debug)]
enum ParseFoldError {
    #[error("invalid fold format (expected \"fold along <axis>=<coordinate>\", got {0})")]
    UnexpectedFormat(String),
    #[error("unknown axis (expected x | y, got {0})")]
    UnknownAxis(String),
    #[error("error parsing coordinate {0}")]
    ParseCoordinate(#[from] ParseIntError),
}

impl FromStr for Fold {
    type Err = ParseFoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(' ').collect();
        if parts.len() != 3 {
            return Err(ParseFoldError::UnexpectedFormat(String::from(s)));
        }
        let axis: Vec<_> = parts[2].split('=').collect();
        if axis.len() != 2 {
            return Err(ParseFoldError::UnexpectedFormat(String::from(s)));
        }
        let coordinate = axis[1].parse()?;
        match axis[0] {
            "x" => Ok(Fold::X(coordinate)),
            "y" => Ok(Fold::Y(coordinate)),
            a => Err(ParseFoldError::UnknownAxis(String::from(a))),
        }
    }
}

fn fold_x(p: &Point, coordinate: i32, max: i32) -> Point {
    if p.x < coordinate {
        *p
    } else if p.x > coordinate {
        let x = coordinate - (max - coordinate) + (max - p.x);
        Point { x, ..*p }
    } else {
        panic!("point on fold line");
    }
}

fn fold_y(p: &Point, coordinate: i32, max: i32) -> Point {
    if p.y < coordinate {
        *p
    } else if p.y > coordinate {
        let y = coordinate - (max - coordinate) + (max - p.y);
        Point { y, ..*p }
    } else {
        panic!("point on fold line");
    }
}

fn main() {
    let inputs: Vec<_> = include_str!("../input")
        .split('\n')
        .fold(Vec::new(), |mut acc, s| {
            if acc.is_empty() {
                acc.push(Vec::new());
            }
            if s == "" {
                acc.push(Vec::new());
            } else {
                acc.last_mut().unwrap().push(s);
            }
            acc
        });
    if inputs.len() < 2 {
        panic!("unexpected input format (expected \"<points>\\n<folds>\")")
    }
    let points: HashSet<Point> = inputs[0].iter().filter_map(|s| s.parse().ok()).collect();
    let folds: Vec<Fold> = inputs[1].iter().filter_map(|s| s.parse().ok()).collect();
    let folding_steps = folds.iter().fold(Vec::new(), |mut acc, f| {
        let pts = acc.last().unwrap_or(&points);
        let next_pts = match f {
            Fold::X(coordinate) => {
                let max = pts.iter().map(|p| p.x).max().expect("points empty");
                pts.iter().map(|p| fold_x(p, *coordinate, max)).collect()
            }
            Fold::Y(coordinate) => {
                let max = pts.iter().map(|p| p.y).max().expect("points empty");
                pts.iter().map(|p| fold_y(p, *coordinate, max)).collect()
            }
        };
        acc.push(next_pts);
        acc
    });
    println!("{:?}", folding_steps[0].len())
}
