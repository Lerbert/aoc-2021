use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug)]
enum ParseDirectionError {
    UnknownDirection(String),
    ParseStride(ParseIntError),
}

impl From<ParseIntError> for ParseDirectionError {
    fn from(e: ParseIntError) -> Self { 
        Self::ParseStride(e)
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        let stride = parts[1].parse::<i32>()?;
        match parts[0] {
            "forward" => Ok(Direction::Forward(stride)),
            "down" => Ok(Direction::Down(stride)),
            "up" => Ok(Direction::Up(stride)),
            d => Err(ParseDirectionError::UnknownDirection(String::from(d)))
        }
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    if let Ok(inputs) = parse_inputs("./input") {
        let final_position = determine_final_position(&inputs, None);
        println!("Final position: {:?} ({})", final_position, final_position.horizontal * final_position.depth);
        let final_position_with_aim = determine_final_position_with_aim(&inputs, None);
        println!("Final position with aim: {:?} ({})", final_position_with_aim, final_position_with_aim.horizontal * final_position_with_aim.depth);
    }
}

fn parse_inputs(filename: &str) -> io::Result<Vec<Direction>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    Ok(
        lines
        .filter_map(|s| {
            s.ok().and_then(|s| s.parse::<Direction>().ok())
        })
        .collect()
    )
}


fn determine_final_position(commands: &Vec<Direction>, start_position: Option<Position>) -> Position {
    let mut position = start_position.unwrap_or(Position{horizontal: 0, depth: 0, aim: 0});
    for direction in commands {
        match direction{
            Direction::Forward(stride) => position.horizontal += stride,
            Direction::Down(stride) => position.depth += stride,
            Direction::Up(stride) => position.depth -= stride,
        }
    }
    position
}

fn determine_final_position_with_aim(commands: &Vec<Direction>, start_position: Option<Position>) -> Position {
    let mut position = start_position.unwrap_or(Position{horizontal: 0, depth: 0, aim:0});
    for direction in commands {
        match direction{
            Direction::Forward(stride) => {
                position.horizontal += stride;
                position.depth += stride * position.aim;
            }
            Direction::Down(stride) => position.aim += stride,
            Direction::Up(stride) => position.aim -= stride,
        }
    }
    position
}
