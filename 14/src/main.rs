use std::collections::HashMap;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseRuleError {
    #[error("invalid rule format (expected [A-Z][A-Z]-[A-Z], got {0})")]
    UnexpectedFormat(String),
}

fn rule_from_str(s: &str) -> Result<((char, char), char), ParseRuleError> {
    let splits: Vec<_> = s.split("->").map(|s| s.trim()).collect();
    if splits.len() != 2 {
        Err(ParseRuleError::UnexpectedFormat(String::from(s)))
    } else {
        let match_pair = splits[0]
            .chars()
            .zip(splits[0].chars().skip(1))
            .next()
            .ok_or(ParseRuleError::UnexpectedFormat(String::from(s)))?;
        let insert = splits[1]
            .chars()
            .next()
            .ok_or(ParseRuleError::UnexpectedFormat(String::from(s)))?;
        Ok((match_pair, insert))
    }
}

fn polymerization_step(template: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    template
        .iter()
        .zip(template.iter().skip(1))
        .map(|(&c1, &c2)| {
            rules
                .get(&(c1, c2))
                .map(|i| vec![c1, *i, c2])
                .unwrap_or(vec![c1, c2])
        })
        .reduce(|mut acc, v| {
            acc.extend(v.iter().skip(1));
            acc
        })
        .expect("empty template")
}

fn polymerization(
    mut template: Vec<char>,
    rules: &HashMap<(char, char), char>,
    steps: u32,
) -> Vec<char> {
    for _ in 0..steps {
        template = polymerization_step(template, rules);
    }
    template
}

fn polymer_score(polymer: &Vec<char>) -> u32 {
    let occurences = polymer.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    occurences.values().max().expect("empty polymer")
        - occurences.values().min().expect("empty polymer")
}

fn main() -> Result<()> {
    let mut inputs = include_str!("../input").split('\n');
    let template: Vec<_> = inputs.next().expect("empty input").chars().collect();
    let rules: HashMap<(char, char), char> = inputs
        .skip(1)
        .filter(|&s| s != "")
        .map(|s| rule_from_str(s))
        .collect::<Result<_, _>>()?;

    let p = polymerization(template, &rules, 10);
    let score = polymer_score(&p);
    println!("{}", score);
    
    Ok(())
}
