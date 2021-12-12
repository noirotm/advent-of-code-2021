use crate::solver::{ReadExt, Solver};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Link>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        // build graph
        let g = build_graph(input);
        let traversals = find_all_traversals(&g);

        traversals.len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        // build graph
        let g = build_graph(input);
        let traversals = find_all_traversals_part2(&g);

        traversals.len()
    }
}

#[allow(unused)]
fn print_traversal(t: &[Node]) {
    println!(
        "{}",
        t.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

#[derive(Clone)]
struct Traversal {
    nodes: Vec<Node>,
    small_visited: HashSet<Node>,
}

fn find_all_traversals(g: &Graph<Node>) -> Vec<Vec<Node>> {
    // queue of nodes to visit
    let mut to_visit = VecDeque::new();
    // final traversals
    let mut traversals = vec![];

    // follow the graph
    to_visit.push_back(Traversal {
        nodes: vec![Node::Start],
        small_visited: HashSet::new(),
    });
    while let Some(node_traversal) = to_visit.pop_front() {
        if let Some(last) = node_traversal.nodes.last() {
            // if we're at the end, add to the traversals, and continue
            if last == &Node::End {
                traversals.push(node_traversal.nodes);
                continue;
            }

            // find all possible subsequent nodes, and add those to the queue
            if let Some(next) = g.nodes.get(last) {
                for node in next {
                    // ignore small nodes if we've seen them before
                    if node_traversal.small_visited.contains(node) {
                        continue;
                    }

                    let mut v = node_traversal.clone();
                    v.nodes.push(node.clone());
                    if matches!(node, Node::Small(_)) {
                        v.small_visited.insert(node.clone());
                    }

                    to_visit.push_back(v);
                }
            }
        }
    }

    traversals
}

#[derive(Clone)]
struct TraversalPart2 {
    nodes: Vec<Node>,
    small_visited: HashMap<Node, usize>,
    small_dup: Option<Node>,
}

fn find_all_traversals_part2(g: &Graph<Node>) -> Vec<Vec<Node>> {
    // queue of nodes to visit
    let mut to_visit = VecDeque::new();
    // final traversals
    let mut traversals = vec![];

    // follow the graph
    to_visit.push_back(TraversalPart2 {
        nodes: vec![Node::Start],
        small_visited: HashMap::new(),
        small_dup: None,
    });
    while let Some(node_traversal) = to_visit.pop_front() {
        if let Some(last) = node_traversal.nodes.last() {
            // if we're at the end, add to the traversals, and continue
            if last == &Node::End {
                traversals.push(node_traversal.nodes);
                continue;
            }

            // find all possible subsequent nodes, and add those to the queue
            if let Some(next) = g.nodes.get(last) {
                for node in next {
                    // ignore small nodes if we've seen them twice before
                    if let Some(dup) = &node_traversal.small_dup {
                        // if we find the 3rd passage of one node, bail
                        // or if we have dupes and we try to dupe another one, bail
                        if dup == node || node_traversal.small_visited.contains_key(node) {
                            continue;
                        }
                    }

                    let mut v = node_traversal.clone();
                    v.nodes.push(node.clone());
                    if matches!(node, Node::Small(_)) {
                        let e = v.small_visited.entry(node.clone()).or_default();
                        *e += 1;
                        if *e == 2 {
                            v.small_dup = Some(node.clone());
                        }
                    }
                    to_visit.push_back(v);
                }
            }
        }
    }

    traversals
}

fn build_graph(links: &[Link]) -> Graph<Node> {
    let mut g = Graph::new();

    for link in links {
        let from = link.from.parse::<Node>().ok();
        let to = link.to.parse::<Node>().ok();

        if let (Some(from), Some(to)) = (from, to) {
            if from != Node::End && to != Node::Start {
                g.add(from.clone(), to.clone());
            }

            if from != Node::Start && to != Node::End {
                g.add(to.clone(), from.clone());
            }
        }
    }

    g
}

pub struct Link {
    from: String,
    to: String,
}

impl FromStr for Link {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('-');
        let from = s.next().ok_or("missing delimiter")?.to_string();
        let to = s.next().ok_or("missing delimiter")?.to_string();

        Ok(Self { from, to })
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Node::Start => "start",
                Node::End => "end",
                Node::Small(s) => s.as_str(),
                Node::Big(s) => s.as_str(),
            }
        )
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            s if s.chars().next().ok_or("empty string")?.is_ascii_uppercase() => {
                Ok(Node::Big(s.to_string()))
            }
            s => Ok(Node::Small(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Graph<T> {
    nodes: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }
}

impl<T> Graph<T>
where
    T: Eq + Hash,
{
    fn add(&mut self, from: T, to: T) {
        let e = self.nodes.entry(from).or_insert_with(HashSet::new);
        e.insert(to);
    }
}
