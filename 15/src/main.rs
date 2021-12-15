use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;

type Index = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Path {
    to_node: Index,
    risk: u32,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // if other has lower risk, we have lower Ordering
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.to_node.cmp(&other.to_node))
    }
}

#[derive(Debug)]
struct Cave {
    risk_levels: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    expansion_factor: usize,
}

impl Cave {
    fn from(risk_levels: Vec<Vec<u8>>, expansion_factor: usize) -> Self {
        Cave {
            height: risk_levels.len(),
            width: risk_levels[0].len(),
            expansion_factor,
            risk_levels,
        }
    }

    fn get_neighbors(&self, node: Index) -> Vec<(Index, u8)> {
        let mut neighbors = Vec::new();
        if node.0 > 0 {
            neighbors.push(((node.0 - 1, node.1), self.get_risk((node.0 - 1, node.1))));
        }
        if node.1 > 0 {
            neighbors.push(((node.0, node.1 - 1), self.get_risk((node.0, node.1 - 1))));
        }
        neighbors.push(((node.0 + 1, node.1), self.get_risk((node.0 + 1, node.1))));
        neighbors.push(((node.0, node.1 + 1), self.get_risk((node.0, node.1 + 1))));
        neighbors
            .iter()
            .filter_map(|&(idx, r)| r.map(|r| (idx, r)))
            .collect()
    }

    fn get_risk(&self, node: Index) -> Option<u8> {
        let risk_incr = node.0 / self.height + node.1 / self.width;
        let base_idx = (node.0 % self.height, node.1 % self.width);
        if node.0 < self.height * self.expansion_factor
            && node.1 < self.width * self.expansion_factor
        {
            // Index is in bounds, 0 is enforced by unsigned type
            Some((self.risk_levels[base_idx.0][base_idx.1] + risk_incr as u8) % 9 + 1)
        } else {
            None
        }
    }
}

fn lowest_risk_cost(cave: &Cave, from: Index, to: Index) -> Option<u32> {
    let mut lowest_risks = HashMap::new();
    let mut pq = BinaryHeap::new();
    pq.push(Path {
        to_node: from,
        risk: 0,
    });

    while let Some(current) = pq.pop() {
        if lowest_risks
            .get(&current.to_node)
            .map(|&p| current > p)
            .unwrap_or(true)
        {
            lowest_risks.insert(current.to_node, current);

            for (next, risk) in cave.get_neighbors(current.to_node) {
                let total_risk = current.risk + risk as u32;
                if lowest_risks
                    .get(&next)
                    .map(|p| p.risk > total_risk)
                    .unwrap_or(true)
                {
                    pq.push(Path {
                        to_node: next,
                        risk: total_risk,
                    })
                }
            }
        }
    }
    lowest_risks.get(&to).map(|p| p.risk)
}

fn find_path(inputs: &Vec<Vec<u8>>, expansion_factor: usize) -> Option<u32> {
    let cave = Cave::from(inputs.clone(), expansion_factor);
    lowest_risk_cost(
        &cave,
        (0, 0),
        (
            cave.height * expansion_factor - 1,
            cave.width * expansion_factor - 1,
        ),
    )
}

fn main() {
    let inputs = include_str!("../input");
    let inputs: Vec<Vec<u8>> = inputs
        .split("\n")
        .filter(|&s| s != "")
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("non number character found") as u8 - 1)
                .collect()
        })
        .collect();
    let risk = find_path(&inputs, 1).expect("no path found");
    println!("Using only the map fragment the lowest risk is {}", risk);

    let risk = find_path(&inputs, 5).expect("no path found");
    println!("Using the full map the lowest risk is {}", risk);
}
