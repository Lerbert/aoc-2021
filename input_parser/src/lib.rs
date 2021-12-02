use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn parse_inputs<T: FromStr>(filename: &str) -> io::Result<Vec<T>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    Ok(
        lines
        .filter_map(|s| {
            s.ok().and_then(|s| s.parse::<T>().ok())
        })
        .collect()
    )
}
