use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let (instructions, nodes) = instructions_and_nodes_from_input(&input);
    let steps = walk_nodes(instructions, nodes);
    println!("{}", steps)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    name: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let santized = value.replace(' ', "").replace('(', "").replace(')', "");
        let (name, lr) = santized.split_once('=').expect("couldn't split on '='");
        let (left, right) = lr.split_once(',').expect("Couln't split on ','");
        Self {
            name: name.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }
}

fn get_node<'a>(name: &str, nodes: &'a Vec<Node>) -> Option<&'a Node> {
    nodes
        .into_iter()
        .position(|n| &n.name == name)
        .and_then(|i| nodes.get(i))
}

fn walk_nodes(instructions: String, nodes: Vec<Node>) -> u64 {
    let mut current_name = "AAA";
    let mut steps = 0;
    while current_name != "ZZZ" {
        instructions.chars().into_iter().for_each(|c| {
            let current_node = get_node(current_name, &nodes).expect("Couln't get node");
            match c {
                'R' => {
                    println!(
                        "Going right\n Current Name: {}\nNew Name: {}",
                        current_name, current_node.right
                    );
                    current_name = &current_node.right;
                }
                'L' => {
                    println!(
                        "Going left\n Current Name: {}\nNew Name: {}",
                        current_name, current_node.left
                    );
                    current_name = &current_node.left;
                }
                _ => unreachable!(),
            }
            steps += 1;
        })
    }
    steps
}

fn instructions_and_nodes_from_input(input: &str) -> (String, Vec<Node>) {
    let mut lines: VecDeque<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let instructions = lines.pop_front().unwrap().to_string();
    let nodes = lines.into_iter().map(|l| Node::from(l)).collect();
    (instructions, nodes)
}

#[cfg(test)]
mod tests {
    use crate::{instructions_and_nodes_from_input, walk_nodes, Node};

    #[test]
    fn get_pt1_steps_correct() {
        let input = "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)";

        let input2 = "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)";

        let (instructions, nodes) = instructions_and_nodes_from_input(input);
        let steps = walk_nodes(instructions, nodes);
        assert_eq!(2, steps);

        let (instructions, nodes) = instructions_and_nodes_from_input(input2);
        println!("SOLVING FOR: {}\n{:?}", instructions, nodes);
        let steps = walk_nodes(instructions, nodes);
        assert_eq!(6, steps);
    }
}
