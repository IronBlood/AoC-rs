use std::collections::VecDeque;

fn build_adj(data: &str) -> Vec<Vec<usize>> {
    data.lines()
        .map(|line| {
            let (_pid, rhs) = line.split_once(" <-> ").expect("bad line");
            rhs.split(", ").map(|s| s.parse().unwrap()).collect()
        })
        .collect()
}

fn count_connected(data: &str) -> usize {
    let adjs = build_adj(data);
    let mut visited = vec![false; adjs.len()];
    let mut queue = VecDeque::from([0usize]);
    let mut count = 0;

    while let Some(u) = queue.pop_front() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        count += 1;
        for &v in &adjs[u] {
            if !visited[v] {
                queue.push_back(v);
            }
        }
    }
    count
}

fn dfs(adjs: &[Vec<usize>], groups: &mut [i32], pid: usize, gid: i32) {
    if groups[pid] >= 0 {
        return;
    }
    groups[pid] = gid;
    for &a in &adjs[pid] {
        dfs(adjs, groups, a, gid);
    }
}

fn count_groups(data: &str) -> u32 {
    let adjs = build_adj(data);
    let mut groups = vec![-1; adjs.len()];
    let mut gid = 0;

    for i in 0..groups.len() {
        if groups[i] < 0 {
            dfs(&adjs, &mut groups, i, gid);
            gid += 1;
        }
    }
    gid as u32
}

pub fn run(input: &str) {
    println!("{}", count_connected(input));
    println!("{}", count_groups(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            .trim();
        assert_eq!(count_connected(data), 6);
        assert_eq!(count_groups(data), 2);
    }
}
