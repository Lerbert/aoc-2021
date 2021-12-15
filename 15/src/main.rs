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

struct Cave {
    risk_levels: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Cave {
    fn from(risk_levels: Vec<Vec<u8>>) -> Self {
        Cave {
            height: risk_levels.len(),
            width: risk_levels[0].len(),
            risk_levels,
        }
    }

    fn get_neighbors(&self, node: Index) -> Vec<(Index, u8)> {
        let mut neighbors = Vec::new();
        if node.0 > 0 {
            neighbors.push(((node.0 - 1, node.1), self.risk_levels[node.0 - 1][node.1]));
        }
        if node.1 > 0 {
            neighbors.push(((node.0, node.1 - 1), self.risk_levels[node.0][node.1 - 1]));
        }
        if node.0 < self.height - 1 {
            neighbors.push(((node.0 + 1, node.1), self.risk_levels[node.0 + 1][node.1]));
        }
        if node.1 < self.width - 1 {
            neighbors.push(((node.0, node.1 + 1), self.risk_levels[node.0][node.1 + 1]));
        }
        neighbors
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
                if lowest_risks.get(&next).map(|p| p.risk > total_risk).unwrap_or(true) {
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

fn main() {
    let inputs = include_str!("../input");
    let inputs: Vec<Vec<u8>> = inputs
        .split("\n")
        .filter(|&s| s != "")
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("non number character found") as u8)
                .collect()
        })
        .collect();
    println!("{:?}", inputs);
    let cave = Cave::from(inputs);
    let r = lowest_risk_cost(&cave, (0, 0), (cave.height - 1, cave.width - 1));
    println!("{:?}", r);
}
