use std::collections::HashMap;

use anyhow::Result;
use thiserror::Error;

#[derive(Clone, Debug)]
struct Polymer {
    pairs: HashMap<(char, char), u128>, // Count available pairs in the polymer
    elements: HashMap<char, u128>, // Count elements in the polymer --> for scoring
}

type ElementPair = (char, char);

#[derive(Error, Debug)]
enum ParseRuleError {
    #[error("invalid rule format (expected [A-Z][A-Z] -> [A-Z], got {0})")]
    UnexpectedFormat(String),
}

fn rule_from_str(s: &str) -> Result<(ElementPair, (ElementPair, ElementPair)), ParseRuleError> {
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
        let new_pairs = ((match_pair.0, insert), (insert, match_pair.1));
        Ok((match_pair, new_pairs))
    }
}

fn polymerization_step(
    polymer: &mut Polymer,
    rules: &HashMap<ElementPair, (ElementPair, ElementPair)>,
) {
    let old_template = polymer.pairs.clone();
    old_template.iter().for_each(|(k, v)| {
        if let Some((p1, p2)) = rules.get(&k) {
            *polymer.pairs.entry(*p1).or_insert(0) += v;
            *polymer.pairs.entry(*p2).or_insert(0) += v;
            *polymer.elements.entry(p1.1).or_insert(0) += v;
            *polymer.pairs.entry(*k).or_insert(0) -= v;
        }
    })
}

fn polymerization(
    polymer: &mut Polymer,
    rules: &HashMap<ElementPair, (ElementPair, ElementPair)>,
    steps: u32,
) {
    for _ in 0..steps {
        polymerization_step(polymer, rules);
    }
}

fn polymer_score(polymer: &Polymer) -> u128 {
    polymer.elements.values().max().expect("empty polymer")
        - polymer.elements.values().filter(|&v| *v > 0).min().expect("empty polymer")
}

fn main() -> Result<()> {
    let mut inputs = include_str!("../input").split('\n');
    
    let template = inputs.next().expect("empty input");
    
    let pairs = template.chars().zip(template.chars().skip(1)).fold(HashMap::new(), |mut acc, (c1, c2)| {
        *acc.entry((c1, c2)).or_insert(0) += 1;
        acc
    });
    let elements = template.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let mut polymer = Polymer { pairs, elements };

    let rules: HashMap<ElementPair, (ElementPair, ElementPair)> = inputs
        .skip(1)
        .filter(|&s| s != "")
        .map(|s| rule_from_str(s))
        .collect::<Result<_, _>>()?;

    // Perform first 10 steps
    polymerization(&mut polymer, &rules, 10);
    let score = polymer_score(&polymer);
    println!("After 10 steps the polymer score is {}", score);
    
    // Perform 30 more steps --> 40 in total
    polymerization(&mut polymer, &rules, 30);
    let score = polymer_score(&polymer);
    println!("After 40 steps the polymer score is {}", score);

    Ok(())
}
