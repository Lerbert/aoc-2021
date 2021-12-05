use std::cmp::max;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use input_parser;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        (self.start.x == self.end.x) | (self.start.y == self.end.y)
    }

    fn is_diagonal(&self) -> bool {
        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;
        y_diff.abs() == x_diff.abs()
    }

    fn covered_points(&self) -> Vec<Point> {
        assert!(self.is_horizontal_or_vertical() | self.is_diagonal());
        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;
        (0..=max(y_diff.abs(), x_diff.abs()))
            .map(|i| Point {
                x: self.start.x + (i * x_diff.signum()),
                y: self.start.y + (i * y_diff.signum()),
            })
            .collect()
    }
}

#[derive(Debug)]
enum ParseLineError {
    ParseCoordinate(ParseIntError),
    UnexpectedFormat(String),
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Result<Vec<i32>, ParseIntError> = s
            .split("->")
            .map(|p| p.trim().split(','))
            .flatten()
            .map(|n| n.parse::<i32>())
            .collect();
        let coordinates = coordinates.map_err(|e| ParseLineError::ParseCoordinate(e))?;
        if coordinates.len() != 4 {
            return Err(ParseLineError::UnexpectedFormat(String::from(s)));
        }
        Ok(Line {
            start: Point {
                x: coordinates[0],
                y: coordinates[1],
            },
            end: Point {
                x: coordinates[2],
                y: coordinates[3],
            },
        })
    }
}

fn main() {
    if let Ok(inputs) = input_parser::parse_inputs::<Line>("./input") {
        let h_v_lines = inputs
            .iter()
            .filter(|&l| l.is_horizontal_or_vertical())
            .collect();
        let covered_points = find_num_points_covered_by_at_least_two_lines(&h_v_lines);
        println!(
            "{} points are covered by at least two horizontal or vertical lines",
            covered_points
        );
        let h_v_d_lines = inputs
            .iter()
            .filter(|&l| l.is_horizontal_or_vertical() | l.is_diagonal())
            .collect();
        let covered_points_diag = find_num_points_covered_by_at_least_two_lines(&h_v_d_lines);
        println!(
            "{} points are covered by at least two horizontal, vertical or diagonal lines",
            covered_points_diag
        );
    }
}

fn find_num_points_covered_by_at_least_two_lines(lines: &Vec<&Line>) -> u32 {
    let mut covered_points: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        let points = line.covered_points();
        for point in points {
            covered_points.insert(
                point,
                covered_points.get(&point).map(|cnt| cnt + 1).unwrap_or(1),
            );
        }
    }
    covered_points.values().filter(|&cnt| *cnt > 1).count() as u32
}
