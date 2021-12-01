use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(inputs) = parse_inputs("./input") {
        let increased = count_increases(&running_sum(&inputs, 3));
        println!("Depth increased {} times", increased);
    }
}

fn parse_inputs(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    Ok(
        lines
        .filter_map(|s| {
            s.ok().and_then(|s| s.parse::<i32>().ok())
        })
        .collect()
    )
}

fn running_sum(depths: &Vec<i32>, window_size: usize) -> Vec<i32> {
    (window_size..depths.len() + 1)
        .map(|i| {
            depths[i - window_size..i].iter().sum()
        })
        .collect()
}

fn count_increases(inputs: &Vec<i32>) -> u32 {
    let mut increased = 0;
    let mut prev_depth : Option<i32> = None;
    for depth in inputs {
        if let Some(pd) = prev_depth {
            if pd < *depth {
                increased += 1;  
            }
        }
        prev_depth = Some(*depth);
    }
    increased
}
