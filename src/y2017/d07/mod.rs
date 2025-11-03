use std::collections::{HashMap, HashSet};

use regex::Regex;

fn get_root(data: &str) -> &str {
    let mut non_roots: HashSet<&str> = HashSet::new();
    let mut candidates: HashSet<&str> = HashSet::new();
    for line in data.lines() {
        if let Some((left, children)) = line.split_once(" -> ") {
            let parent = left.split_whitespace().next().unwrap();
            if !non_roots.contains(parent) {
                candidates.insert(parent);
            }

            for c in children.split(", ") {
                non_roots.insert(c);
                candidates.remove(c);
            }
        } else {
            let node = line.split_whitespace().next().unwrap();
            non_roots.insert(node);
        }
    }

    candidates.into_iter().next().expect("no unique root")
}

struct Tower {
    weight: u32,
    parent: Option<usize>,
    children: Vec<usize>,
    sum: u32,
}

fn dfs(nodes: &mut [Tower], idx: usize, wrong: &mut Option<usize>) -> u32 {
    let (weight, children) = {
        let node = &nodes[idx];
        (node.weight, node.children.clone())
    };

    let total: u32 = weight
        + children
            .iter()
            .copied()
            .map(|c| dfs(nodes, c, wrong))
            .sum::<u32>();
    nodes[idx].sum = total;

    if wrong.is_none() && children.len() > 1 {
        let mut freq = HashMap::new();
        for &c in &children {
            let sum = nodes[c].sum;
            *freq.entry(sum).or_insert(0usize) += 1;
        }
        if freq.len() > 1 {
            let (&bad_sum, _) = freq.iter().find(|(_, c)| **c == 1).unwrap();
            let bad_idx = children
                .iter()
                .position(|&c| nodes[c].sum == bad_sum)
                .unwrap();
            *wrong = Some(children[bad_idx]);
        }
    }

    total
}

fn find_wrong_weight(data: &str) -> u32 {
    let regex_node = Regex::new(r"(\w+) \((\d+)\)").unwrap();
    let lines: Vec<_> = data.lines().collect();
    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();

    let mut nodes = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let caps = regex_node.captures(line).expect("no match");
        let name = caps.get(1).unwrap().as_str();
        let weight: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        name_to_idx.insert(name, i);
        nodes.push(Tower {
            weight,
            parent: None,
            children: Vec::new(),
            sum: 0,
        });
    }

    for &line in &lines {
        let arr: Vec<_> = line.split(" -> ").collect();
        if arr.len() == 2 {
            let parent_name = arr[0].split_whitespace().next().unwrap();
            let parent_idx = name_to_idx.get(&parent_name).unwrap();
            let children: Vec<usize> = arr[1]
                .split(", ")
                .map(|name| {
                    if let Some(idx) = name_to_idx.get(name) {
                        if let Some(node) = nodes.get_mut(*idx) {
                            node.parent = Some(*parent_idx);
                            return *idx;
                        }
                    }
                    panic!("invalid input");
                })
                .collect();

            let parent = nodes.get_mut(*parent_idx).unwrap();
            parent.children = children;
        }
    }

    let root_idx = nodes
        .iter()
        .position(|n| n.parent.is_none())
        .expect("no root");

    let mut wrong = None;
    dfs(&mut nodes, root_idx, &mut wrong);

    let wrong = wrong.expect("no wrong nodes");
    let p = nodes[wrong].parent.unwrap();
    let mut correct: u32 = 0;
    let p = &nodes[p];
    for &c in &p.children {
        if nodes[c].sum != nodes[wrong].sum {
            correct = nodes[c].sum;
            break;
        }
    }

    correct + nodes[wrong].weight - nodes[wrong].sum
}

pub fn run(input: &str) {
    println!("{}", get_root(input));
    println!("{}", find_wrong_weight(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"
            .trim();
        assert_eq!(get_root(data), "tknk");
        assert_eq!(find_wrong_weight(data), 60);
    }
}
