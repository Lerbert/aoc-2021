use std::str::FromStr;
use std::num::ParseIntError;

use input_parser;

#[derive(Clone, Copy, Debug)]
enum MarkedNumber {
    Marked(u32),
    Unmarked(u32),
}

impl FromStr for MarkedNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MarkedNumber::Unmarked(s.parse::<u32>()?))
    }
}

#[derive(Debug)]
struct Board {
    fields: [[MarkedNumber; 5]; 5]
}

impl Board {
    fn new(repr: &[String]) -> Result<Self, ParseBoardError> {
        let repr: Vec<Vec<MarkedNumber>> = repr.iter()
            .map(|s| {
                s.split(' ')
                    .filter_map(|n| {
                        match n {
                            "" => None,
                            number_str => number_str.parse::<MarkedNumber>().ok()
                        }
                    })
                    .collect()
            })
            .collect();
        if repr.len() != 5 {
            return Err(ParseBoardError::UnexpectedLength(repr.len()));
        }
        let mut fields = [[MarkedNumber::Unmarked(0); 5]; 5];
        for (i, row) in repr.iter().enumerate() {
            if row.len() != 5 {
                return Err(ParseBoardError::UnexpectedWidth(row.len()));
            }
            for (j, number) in row.iter().enumerate() {
                fields[i][j] = *number;
            }
        }
        Ok(Board{fields})
    }

    fn mark(&mut self, number: u32) {
        for i in 0..5 {
            for j in 0..5 {
                if let MarkedNumber::Unmarked(x) = self.fields[i][j] {
                    if x == number {
                        self.fields[i][j] = MarkedNumber::Marked(x)
                    }
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        let mut won = false;
        for i in 0..5 {
            let mut row_fully_marked = true;
            let mut col_fully_marked = true;
            for j in 0..5 {
                row_fully_marked &= matches!(self.fields[i][j], MarkedNumber::Marked(_));
                col_fully_marked &= matches!(self.fields[j][i], MarkedNumber::Marked(_));
            }
            won |= row_fully_marked | col_fully_marked;
        }
        won
    }

    fn score(&self) -> u32 {
        self.fields.iter()
            .map(|row| row.iter())
            .flatten()
            .filter_map(|n| {
                match n {
                    MarkedNumber::Unmarked(x) => Some(x),
                    _ => None,
                }
            })
            .sum()
    }
}

#[derive(Debug)]
enum ParseBoardError {
    UnexpectedWidth(usize),
    UnexpectedLength(usize),
}

fn main() {
    if let Ok(inputs) = input_parser::parse_inputs::<String>("./input") {
        let mut split_iter = inputs.split(|s| s == "");
        let drawn_numbers = split_iter.next().expect("no drawn numbers given");
        if drawn_numbers.len() != 1 {
            panic!("unexpected format for drawn numbers")
        }
        let drawn_numbers: Vec<u32> = drawn_numbers[0].split(',').filter_map(|s| s.parse::<u32>().ok()).collect();
        let mut boards: Vec<Board> = split_iter.filter_map(|l| Board::new(l).ok()).collect();
        let (first_score, last_score) = play_bingo(&drawn_numbers, &mut boards);
        println!("First winning board has score {}", first_score);
        println!("Last winning board has score {}", last_score);
    }
}

fn play_bingo(drawn_numbers: &Vec<u32>, boards: &mut Vec<Board>) -> (u32, u32) {
    let mut winning_boards: Vec<u32> = Vec::new();
    for number in drawn_numbers {
        if boards.is_empty() {
            break;
        }
        for board in boards.iter_mut() {
            board.mark(*number)
        }
        winning_boards.extend(boards.iter().filter(|b| b.has_won()).map(|b| b.score() * number));
        boards.retain(|b| !b.has_won());
    }
    (winning_boards[0], winning_boards[winning_boards.len() - 1])
}
