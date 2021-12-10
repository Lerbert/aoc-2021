#[derive(Debug)]
enum ParseChunkError {
    MissingToken(char),
    UnexpectedToken(char, char, u32),
}

fn parse_chunk(chunk: &str) -> Result<(), ParseChunkError> {
    // This is a CFL, so as it can be parsed with a PDA using one stack is enough
    // Grammar: S ::= epsilon | (S) | [S] | {S} | <S>
    let mut stack = Vec::new();
    for c in chunk.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            x => {
                match stack.pop().and_then(|c| matching_bracket(c)) {
                    Some(y) if y == x => (),
                    Some(z) => return Err(ParseChunkError::UnexpectedToken(z, x, score(x))),
                    None => return Err(ParseChunkError::UnexpectedToken('e', x, score(x))),
                }
            },
        }
    }
    Ok(())
}

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
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
    let total_score: u32 = inputs.iter().map(|chunk| parse_chunk(chunk)).filter_map(|r| {
        match r {
            Ok(_) => None,
            Err(ParseChunkError::UnexpectedToken(expected, got, score)) => Some(score),
            Err(_) => None
        }
    }).sum();
    println!("{:?}", total_score);
}
