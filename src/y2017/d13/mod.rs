// defined by ChatGPT
type Entry = (u32, u32);

fn parse(data: &str) -> Vec<Entry> {
    data.lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").expect("bad line");
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn _get_severity(cfgs: &[Entry], delays: u32) -> u32 {
    cfgs.iter()
        .map(|&(id, depth)| {
            if (id + delays) % ((depth - 1) * 2) == 0 {
                id * depth
            } else {
                0
            }
        })
        .sum()
}

fn get_severity(data: &str) -> u32 {
    _get_severity(&parse(data), 0)
}

fn get_delays(data: &str) -> u32 {
    let cfgs = parse(data);

    // polished by ChatGPT
    (0u32..)
        .find(|&d| {
            cfgs.iter().all(|&(id, depth)| {
                let p = 2 * (depth - 1);
                p != 0 && (id + d) % p != 0
            })
        })
        .unwrap()
}

pub fn run(input: &str) {
    println!("{}", get_severity(input));
    println!("{}", get_delays(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "
0: 3
1: 2
4: 4
6: 4"
            .trim();
        assert_eq!(get_severity(data), 24);
        assert_eq!(get_delays(data), 10);
    }
}
