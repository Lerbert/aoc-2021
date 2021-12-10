#[derive(Debug)]
enum ParseChunkError {
    MissingTokens(Vec<char>),
    UnexpectedToken(char, char, u32),
}

fn parse_chunk(chunk: &str) -> Result<(), ParseChunkError> {
    // This is a CFL, so as it can be parsed with a PDA using one stack is enough
    // Grammar: S ::= epsilon | (S) | [S] | {S} | <S>
    let mut stack = Vec::new();
    for c in chunk.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            x => match stack.pop().and_then(|c| matching_bracket(c)) {
                Some(y) if y == x => (),
                Some(z) => return Err(ParseChunkError::UnexpectedToken(z, x, unexpected_score(x))),
                None => {
                    return Err(ParseChunkError::UnexpectedToken(
                        'e',
                        x,
                        unexpected_score(x),
                    ))
                }
            },
        }
    }
    if !stack.is_empty() {
        Err(ParseChunkError::MissingTokens(
            stack
                .iter()
                .filter_map(|&c| matching_bracket(c))
                .rev()
                .collect(),
        ))
    } else {
        Ok(())
    }
}

fn unexpected_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn missing_score(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn matching_bracket(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn main() {
    let inputs: Vec<_> = include_str!("../input").split('\n').collect();
    let parse_results: Vec<_> = inputs.iter().map(|chunk| parse_chunk(chunk)).collect();
    let total_unexpected_score: u32 = parse_results
        .iter()
        .filter_map(|r| match r {
            Ok(_) => None,
            Err(ParseChunkError::UnexpectedToken(_expected, _got, score)) => Some(score),
            Err(_) => None,
        })
        .sum();
    let mut missing_scores: Vec<_> = parse_results
        .iter()
        .filter_map(|r| match r {
            Ok(_) => None,
            Err(ParseChunkError::MissingTokens(tokens)) => Some(
                tokens
                    .iter()
                    .map(|&c| missing_score(c))
                    .fold(0, |acc, s| acc * 5 + s as u128),
            ),
            Err(_) => None,
        })
        .collect();
    missing_scores.sort();
    let median_missing_score = missing_scores[missing_scores.len() / 2];
    println!("Total unexpected score: {}", total_unexpected_score);
    println!("Median missing score:   {}", median_missing_score);
}
