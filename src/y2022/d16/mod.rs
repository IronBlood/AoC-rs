// Credit https://github.com/yzhong52/AdventOfCode/blob/master/2022/src/day16.rs
//        https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j4evhg9/
use std::{collections::HashMap, time::Instant};

use regex::Regex;

type Valve = usize;
#[derive(Debug, Clone)]
struct ValveProp {
    rate: i32,
    leading_valves: Vec<Valve>,
}

fn parse(data: &str) -> HashMap<Valve, ValveProp> {
    let re: Regex =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .unwrap();

    let mut result: HashMap<Valve, ValveProp> = HashMap::new();

    let mut name_to_index: HashMap<&str, Valve> = [("AA", 0)].into_iter().collect();

    for line in data.split("\n") {
        let capture = re
            .captures(line.trim())
            .expect(&format!("Unable to unwrap string '{}'", line));

        let source_str: &str = capture.get(1).unwrap().as_str();
        if !name_to_index.contains_key(source_str) {
            name_to_index.insert(source_str, name_to_index.len());
        }

        let source: Valve = *name_to_index.get(source_str).unwrap();
        let rate: i32 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let leading_valves: Vec<Valve> = capture
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|leading_valve_str| {
                if !name_to_index.contains_key(leading_valve_str) {
                    name_to_index.insert(leading_valve_str, name_to_index.len());
                }
                *name_to_index.get(leading_valve_str).unwrap()
            })
            .collect();

        let valve_prop = ValveProp {
            rate,
            leading_valves,
        };
        result.insert(source, valve_prop);
    }

    result
}

fn floyd_warshall(all_valves: &HashMap<Valve, ValveProp>) -> Vec<Vec<usize>> {
    let n = all_valves.len();

    let mut valve_distances = vec![vec![usize::MAX / 2; n]; n];

    for (valve, valve_prop) in all_valves {
        for leading_valve in &valve_prop.leading_valves {
            valve_distances[*valve][*leading_valve] = 1;
            valve_distances[*leading_valve][*valve] = 1;
        }
    }

    for i in 0..n {
        valve_distances[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                valve_distances[i][j] =
                    valve_distances[i][j].min(valve_distances[i][k] + valve_distances[k][j]);
            }
        }
    }

    valve_distances
}

fn solve(
    all_valves: &HashMap<Valve, ValveProp>,
    valve_distances: &Vec<Vec<usize>>,
    cache: &mut HashMap<(Valve, Vec<Valve>, i32), i32>,
    current_valve: Valve,
    pending_valves: Vec<Valve>,
    remaining_steps: i32,
) -> i32 {
    let mut result = 0;
    let key = (current_valve, pending_valves.clone(), remaining_steps);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    for i in 0..pending_valves.len() {
        let next_pending_valve = pending_valves[i];
        let steps = valve_distances[current_valve][next_pending_valve];
        let next_remaining_steps = remaining_steps - (steps as i32) - 1;

        if next_remaining_steps > 0 {
            let next_pending_valves = pending_valves[0..i]
                .to_vec()
                .into_iter()
                .chain(pending_valves[i + 1..].to_vec())
                .collect();

            let next_result = solve(
                all_valves,
                valve_distances,
                cache,
                next_pending_valve,
                next_pending_valves,
                next_remaining_steps,
            );

            let current_result = next_result
                + all_valves.get(&next_pending_valve).unwrap().rate * next_remaining_steps;

            result = result.max(current_result);
        }
    }

    cache.insert(key, result);
    result
}

fn partitions(values: Vec<Valve>) -> Vec<(Vec<Valve>, Vec<Valve>)> {
    let mut result: Vec<Vec<Valve>> = vec![vec![values[0].clone()]];

    for value in &values[1..] {
        let mut new_result = result.clone();
        for r in result {
            let mut cloned = r.clone();
            cloned.push(value.clone());
            new_result.push(cloned);
        }
        result = new_result;
    }

    result
        .into_iter()
        .map(|left| {
            let values_cloned = values.clone();
            let right = values_cloned
                .into_iter()
                .filter(|x| !left.contains(x))
                .collect();
            (left, right)
        })
        .collect()
}

pub fn run(input: &str) {
    let all_valves = parse(input);
    let valves_with_positive_flow_rate: Vec<Valve> = all_valves
        .clone()
        .into_iter()
        .filter(|(_value_id, value_property)| value_property.rate > 0)
        .map(|(value_id, _value_property)| value_id)
        .collect();

    let valve_distances = floyd_warshall(&all_valves);

    let mut cache: HashMap<(Valve, Vec<Valve>, i32), i32> = HashMap::new();

    let start = Instant::now();
    let part1 = solve(
        &all_valves,
        &valve_distances,
        &mut cache,
        0,
        valves_with_positive_flow_rate.clone(),
        30,
    );
    let duration = start.elapsed();
    println!("part1 time elapsed: {:?}", duration);
    println!("part1 {}", part1);

    let start = Instant::now();
    let part2 = partitions(valves_with_positive_flow_rate)
        .into_iter()
        .map(|(left, right)| {
            let left = solve(&all_valves, &valve_distances, &mut cache, 0, left, 26);
            let right = solve(&all_valves, &valve_distances, &mut cache, 0, right, 26);
            left + right
        })
        .max()
        .unwrap();
    let duration = start.elapsed();
    println!("part2 time elapsed: {:?}", duration);
    println!("part2 {}", part2);
}
