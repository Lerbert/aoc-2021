use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use thiserror::Error;

use input_parser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    name: String,
    large: bool,
}

impl Node {
    fn start() -> Self {
        Node {
            name: String::from("start"),
            large: false,
        }
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }
}

#[derive(Debug)]
struct Edge(Node, Node);

#[derive(Error, Debug)]
enum ParseEdgeError {
    #[error("invalid edge format (expected [a-zA-Z]+-[a-zA-Z]+, got {0}")]
    UnexpectedFormat(String),
}

impl FromStr for Edge {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split('-').collect();
        if splits.len() != 2 {
            Err(ParseEdgeError::UnexpectedFormat(String::from(s)))
        } else {
            Ok(Edge(
                Node {
                    name: String::from(splits[0]),
                    large: char::is_uppercase(
                        splits[0].chars().nth(0).expect("no node name given"),
                    ),
                },
                Node {
                    name: String::from(splits[1]),
                    large: char::is_uppercase(
                        splits[1].chars().nth(0).expect("no node name given"),
                    ),
                },
            ))
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<Node>,
    edges: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn from(edges: Vec<Edge>) -> Self {
        let mut edge_map = HashMap::new();
        let mut nodes = HashSet::new();
        for Edge(n1, n2) in edges {
            nodes.insert(n1.clone());
            nodes.insert(n2.clone());
            let connections_n1 = edge_map.entry(n1.clone()).or_insert(HashSet::new());
            connections_n1.insert(n2.clone());
            let connections_n2 = edge_map.entry(n2).or_insert(HashSet::new());
            connections_n2.insert(n1);
        }
        Graph {
            edges: edge_map,
            nodes,
        }
    }

    fn find_paths(&self) -> Vec<Vec<Node>> {
        let mut stack = vec![Node::start()];
        let mut paths = Vec::new();
        self.dfs_helper(&mut stack, &mut paths);
        paths
    }

    fn dfs_helper(&self, stack: &mut Vec<Node>, paths: &mut Vec<Vec<Node>>) {
        let node = stack.last().expect("stack is empty");
        if node.is_end() {
            // Complete a path
            paths.push(stack.clone());
        } else {
            let outgoing = self.edges.get(&node).expect("missing edges");
            let to_visit: Vec<_> = outgoing
                .iter()
                .filter(|n| n.large | !stack.contains(n))
                .collect();
            for node in to_visit {
                stack.push(node.clone());
                self.dfs_helper(stack, paths);
            }
        }
        stack.pop();
    }
}

fn main() {
    if let Ok(edges) = input_parser::parse_inputs::<Edge>("./input") {
        let caves = Graph::from(edges);
        let paths = caves.find_paths();
        println!("{}", paths.len())
    }
}
