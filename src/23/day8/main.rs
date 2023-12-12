use std::{
    collections::{BTreeMap, HashMap, LinkedList},
    fs, u32::MAX,
};

use num::Integer;

fn part1() -> u32 {
    let file = fs::read_to_string("src/23/day8/input.txt").unwrap();
    let (instructions, nodes) = file.split_once("\n\n").unwrap();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for node in nodes.split('\n') {
        if node == "" {
            continue;
        }
        let (id, rest) = node.split_once(" = (").unwrap();
        let (l, mut r) = rest.split_once(", ").unwrap();
        r = r.trim_end_matches(')');

        node_map.insert(id, (l, r));
    }

    let mut current = "AAA";
    let mut pair = node_map.get(current).unwrap();
    let mut count = 0;
    loop {
        for c in instructions.chars() {
            if current == "ZZZ" {
                return count;
            }
            match c {
                'L' => {
                    current = pair.0;
                    pair = node_map.get(current).unwrap();
                }
                'R' => {
                    current = pair.1;
                    pair = node_map.get(current).unwrap();
                }
                _ => panic!("unsupported direction"),
            }
            count = count + 1;
        }
    }
}

fn find_steps(node: &str, node_map: &HashMap<&str, (&str, &str)>, instructions: &str) -> u32 {
    let mut current = node;
    let mut pair = node_map.get(current).unwrap();
    let mut count = 0;
    loop {
        for c in instructions.chars() {
            if current.chars().last().unwrap() == 'Z' {
                dbg!(count);
                return count;
            }
            match c {
                'L' => {
                    current = pair.0;
                    pair = node_map.get(current).unwrap();
                }
                'R' => {
                    current = pair.1;
                    pair = node_map.get(current).unwrap();
                }
                _ => panic!("unsupported direction"),
            }
            count = count + 1;
        }
    }
}

fn part2() -> u64 {
    let file = fs::read_to_string("src/23/day8/input.txt").unwrap();
    let (instructions, nodes) = file.split_once("\n\n").unwrap();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for node in nodes.split('\n') {
        if node == "" {
            continue;
        }
        let (id, rest) = node.split_once(" = (").unwrap();
        let (l, mut r) = rest.split_once(", ").unwrap();
        r = r.trim_end_matches(')');

        node_map.insert(id, (l, r));
    }
    let starts: Vec<_> = node_map
        .iter()
        .filter(|(k, _)| k.chars().last().unwrap() == 'A')
        .collect();
    dbg!(&starts);

    let mut total: u64 = 21389;
    for start in starts {
        total = total.lcm(&(find_steps(start.0, &node_map, instructions) as u64));
    }
    total
}


fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}
