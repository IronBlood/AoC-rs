use std::collections::HashMap;

struct Component {
    idx: usize,
    ports: Vec<u32>,
}

fn add_id_to_map(map: &mut HashMap<u32, Vec<usize>>, port: u32, idx: usize) {
    // ChatGPT or_default
    map.entry(port).or_default().push(idx);
}

fn dfs<F, G>(
    components: &[Component],
    map: &HashMap<u32, Vec<usize>>,
    visited: &mut [u8],
    part: u8,
    curr_id: usize,
    port: u32,
    cb1: &mut F,
    cb2: &mut G,
) where
    F: FnMut(u32),
    G: FnMut(u32, u32),
{
    if visited[curr_id] == 1 {
        if part == 1 {
            let mut sum = 0;
            for c in components.iter() {
                if visited[c.idx] == 1 {
                    sum += c.ports[0] + c.ports[1];
                }
            }
            cb1(sum);
        } else {
            let length = visited.iter().fold(0u32, |acc, &x| acc + x as u32);
            let mut strength = 0;
            for c in components.iter() {
                if visited[c.idx] == 1 {
                    strength += c.ports[0] + c.ports[1];
                }
            }
            cb2(length, strength);
        }
        return;
    }

    visited[curr_id] = 1;
    for cid in map.get(&port).unwrap() {
        let c = &components[*cid];
        let p = if c.ports[0] != port {
            c.ports[0]
        } else {
            c.ports[1]
        };
        dfs(components, map, visited, part, *cid, p, cb1, cb2);
    }
    visited[curr_id] = 0;
}

fn build_bridge(data: &str, part: u8) -> u32 {
    let mut components = Vec::new();
    for (idx, line) in data.lines().enumerate() {
        let ports: Vec<_> = line
            .split('/')
            .map(|x| x.parse::<u32>().expect("not a valid number"))
            .collect();
        components.push(Component { idx, ports });
    }

    let mut map = HashMap::new();
    for c in components.iter() {
        add_id_to_map(&mut map, c.ports[0], c.idx);
        add_id_to_map(&mut map, c.ports[1], c.idx);
    }

    let mut max_p1 = 0;
    let mut max_p2 = 0;
    let mut longest = 0;
    let mut visited = vec![0u8; components.len()];

    let mut cb1 = |x: u32| max_p1 = max_p1.max(x);
    let mut cb2 = |length: u32, strength: u32| {
        if length > longest {
            longest = length;
            max_p2 = strength;
        } else if length == longest {
            max_p2 = max_p2.max(strength);
        }
    };

    for bid in map.get(&0).unwrap() {
        let b = &components[*bid];
        let p = if b.ports[0] != 0 {
            b.ports[0]
        } else {
            b.ports[1]
        };
        dfs(
            &components,
            &map,
            &mut visited,
            part,
            *bid,
            p,
            &mut cb1,
            &mut cb2,
        );
    }

    if part == 1 { max_p1 } else { max_p2 }
}

pub fn run(input: &str) {
    println!("{}", build_bridge(input, 1));
    println!("{}", build_bridge(input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(build_bridge(data, 1), 31);
        assert_eq!(build_bridge(data, 2), 19);
    }
}
