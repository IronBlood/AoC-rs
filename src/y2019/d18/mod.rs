// credit https://github.com/prscoelho/aoc2019/blob/master/src/aoc18/mod.rs
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn neighbours(&self) -> [Coordinate; 4] {
        [
            Coordinate(self.0 - 1, self.1),
            Coordinate(self.0 + 1, self.1),
            Coordinate(self.0, self.1 - 1),
            Coordinate(self.0, self.1 + 1),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Node(char),
}

#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    node: char,
    keys: BTreeSet<char>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
struct FourState {
    steps: usize,
    robots: [char; 4],
    keys: BTreeSet<char>,
}

impl Ord for FourState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for FourState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
struct DijkstraState {
    cost: usize,
    current: char,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.current.cmp(&other.current))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(input: &str) {
    println!("{}", solve_first(input));
    println!("{}", solve_second(input));
}

fn solve_first(input: &str) -> usize {
    let grid = parse_grid(input);
    let graph = graph(&grid);

    search(graph)
}

fn solve_second(input: &str) -> usize {
    let mut grid = parse_grid(input);
    four_robots(&mut grid);
    let graph = graph(&grid);

    search_four(graph)
}

fn parse_grid(input: &str) -> HashMap<Coordinate, Tile> {
    let mut grid = HashMap::new();
    let mut height = 0;
    for line in input.lines() {
        let mut width = 0;
        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                _ => Tile::Node(c),
            };
            grid.insert(Coordinate(width, height), tile);
            width += 1;
        }
        height += 1;
    }
    grid
}

// Change spaces to walls, and create four new robots
fn four_robots(grid: &mut HashMap<Coordinate, Tile>) {
    let robot_coord = grid
        .iter()
        .find(|(_, v)| **v == Tile::Node('@'))
        .map(|(k, _)| k.clone())
        .unwrap();

    grid.insert(robot_coord, Tile::Wall);
    for &neighbour in &robot_coord.neighbours() {
        grid.insert(neighbour, Tile::Wall);
    }

    grid.insert(
        Coordinate(robot_coord.0 - 1, robot_coord.1 - 1),
        Tile::Node('@'),
    );
    grid.insert(
        Coordinate(robot_coord.0 - 1, robot_coord.1 + 1),
        Tile::Node('='),
    );
    grid.insert(
        Coordinate(robot_coord.0 + 1, robot_coord.1 + 1),
        Tile::Node('%'),
    );
    grid.insert(
        Coordinate(robot_coord.0 + 1, robot_coord.1 - 1),
        Tile::Node('$'),
    );
}

fn graph(grid: &HashMap<Coordinate, Tile>) -> HashMap<char, HashMap<char, usize>> {
    let mut graph = HashMap::new();

    for (coord, tile) in grid.iter() {
        if let Tile::Node(c) = tile {
            let pos_edges = reachable_from(grid, *coord);
            graph.insert(*c, pos_edges);
        }
    }

    graph
}

fn reachable_from(grid: &HashMap<Coordinate, Tile>, coord: Coordinate) -> HashMap<char, usize> {
    let mut visited = HashSet::new();
    let mut result = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((coord, 0));
    visited.insert(coord);

    while let Some((curr, steps)) = queue.pop_front() {
        for neighbour in &curr.neighbours() {
            if let Some(tile) = grid.get(neighbour) {
                if !visited.contains(neighbour) {
                    visited.insert(*neighbour);
                    match tile {
                        Tile::Empty => {
                            queue.push_back((*neighbour, steps + 1));
                        }
                        Tile::Node(c) => {
                            result.insert(*c, steps + 1);
                        }
                        Tile::Wall => {}
                    }
                }
            }
        }
    }
    result
}

fn search_keys(
    graph: &HashMap<char, HashMap<char, usize>>,
    keys: &BTreeSet<char>,
    start: char,
) -> Vec<(char, usize)> {
    let mut dist = HashMap::new();
    for &key in graph.keys() {
        dist.insert(key, usize::max_value());
    }

    let mut heap: BinaryHeap<DijkstraState> = BinaryHeap::new();
    *dist.get_mut(&start).unwrap() = 0;
    heap.push(DijkstraState {
        cost: 0,
        current: start,
    });
    let mut reach = HashSet::new();

    while let Some(DijkstraState { cost, current }) = heap.pop() {
        if current.is_lowercase() && !keys.contains(&current) {
            reach.insert(current);
            continue;
        }

        if cost > dist[&current] {
            continue;
        }

        for (&next_node, &next_cost) in graph.get(&current).unwrap().iter() {
            if next_node.is_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                continue;
            }

            let next = DijkstraState {
                cost: cost + next_cost,
                current: next_node,
            };

            if next.cost < dist[&next_node] {
                dist.insert(next_node, next.cost);
                heap.push(next);
            }
        }
    }

    reach.into_iter().map(|node| (node, dist[&node])).collect()
}

fn search(graph: HashMap<char, HashMap<char, usize>>) -> usize {
    let mut priority_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    let mut distances: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();
    distances.insert(('@', BTreeSet::new()), 0);

    let start = State {
        steps: 0,
        node: '@',
        keys: BTreeSet::new(),
    };

    priority_queue.push(start);

    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(curr) = priority_queue.pop() {
        if curr.keys.len() == key_count {
            return curr.steps;
        }

        if let Some(&best_step) = distances.get(&(curr.node, curr.keys.clone())) {
            if curr.steps > best_step {
                continue;
            }
        }

        let cache_key = (curr.node, curr.keys.clone());

        let cached_entry = cache
            .entry(cache_key)
            .or_insert_with(|| search_keys(&graph, &curr.keys, curr.node));

        for &(next_node, cost) in cached_entry.iter() {
            let mut next_keys = curr.keys.clone();
            next_keys.insert(next_node);
            let next_steps = curr.steps + cost;

            let distances_entry = distances
                .entry((next_node, next_keys.clone()))
                .or_insert(usize::max_value());

            if next_steps < *distances_entry {
                *distances_entry = next_steps;

                let next_state = State {
                    steps: curr.steps + cost,
                    node: next_node,
                    keys: next_keys,
                };

                priority_queue.push(next_state);
            }
        }
    }

    usize::max_value()
}

fn search_four(graph: HashMap<char, HashMap<char, usize>>) -> usize {
    let mut priority_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    let mut distances: HashMap<([char; 4], BTreeSet<char>), usize> = HashMap::new();
    let robots = ['@', '=', '%', '$'];

    distances.insert((robots.clone(), BTreeSet::new()), 0);

    let start = FourState {
        steps: 0,
        robots: robots,
        keys: BTreeSet::new(),
    };

    priority_queue.push(start);

    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(curr) = priority_queue.pop() {
        if curr.keys.len() == key_count {
            return curr.steps;
        }

        if let Some(&best_steps) = distances.get(&(curr.robots, curr.keys.clone())) {
            if curr.steps > best_steps {
                continue;
            }
        }

        for (robot_number, &robot_location) in curr.robots.iter().enumerate() {
            let cache_key = (robot_location, curr.keys.clone());
            let cache_entry = cache
                .entry(cache_key)
                .or_insert_with(|| search_keys(&graph, &curr.keys, robot_location));

            for &(next_node, cost) in cache_entry.iter() {
                let mut next_keys = curr.keys.clone();
                next_keys.insert(next_node);

                let mut next_robots = curr.robots.clone();
                next_robots[robot_number] = next_node;

                let next_steps = curr.steps + cost;

                let distances_entry = distances
                    .entry((next_robots.clone(), next_keys.clone()))
                    .or_insert(usize::max_value());

                if next_steps < *distances_entry {
                    *distances_entry = next_steps;
                    let next_state = FourState {
                        steps: next_steps,
                        robots: next_robots,
                        keys: next_keys,
                    };

                    priority_queue.push(next_state);
                }
            }
        }
    }

    usize::max_value()
}
