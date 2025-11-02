use std::collections::HashMap;

enum Command {
    SPIN(u32),
    EXCHANGE(usize, usize),
    PARTNER(u8, u8),
}

fn parse(cmd: &str) -> Command {
    let first = cmd.as_bytes()[0];
    let rest = &cmd[1..];
    match first {
        b's' => Command::SPIN(rest.parse().unwrap()),
        b'x' => {
            let (i, j) = rest.split_once("/").expect("bad input");
            Command::EXCHANGE(i.parse().unwrap(), j.parse().unwrap())
        }
        b'p' => {
            let b = rest.as_bytes();
            assert!(b.len() == 3 && b[1] == b'/');
            Command::PARTNER(b[0], b[2])
        }
        _ => unreachable!("never"),
    }
}

fn spin_helper<T>(arr: &mut [T], mut l: usize, mut r: usize) {
    while l < r {
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }
}

fn handle(arr: &mut [u8], cmd: &Command) {
    match cmd {
        Command::SPIN(x) => {
            spin_helper(arr, 0, arr.len() - (*x as usize) - 1);
            spin_helper(arr, arr.len() - (*x as usize), arr.len() - 1);
            spin_helper(arr, 0, arr.len() - 1);
        }
        Command::EXCHANGE(i, j) => arr.swap(*i, *j),
        Command::PARTNER(i, j) => {
            let i = arr.iter().position(|x| *x == *i).unwrap();
            let j = arr.iter().position(|x| *x == *j).unwrap();
            arr.swap(i, j);
        }
    }
}

fn jump_with_cycle(mut curr: Vec<u8>, total: usize, cmds: &[Command]) -> Vec<u8> {
    let mut seen: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut states: Vec<Vec<u8>> = Vec::new();
    let mut step = 0;

    while step < total {
        if let Some(&first) = seen.get(&curr) {
            let cycle = step - first;
            let remain = (total - step) % cycle;
            return states[first + remain].clone();
        }

        seen.insert(curr.clone(), step);
        states.push(curr.clone());

        for cmd in cmds {
            handle(&mut curr, cmd);
        }

        step += 1;
    }

    curr
}

fn after_dance(moves: &str, str: &str, part: u8) -> String {
    let cmds: Vec<_> = moves.split(",").map(|str| parse(str)).collect();

    let total = match part {
        1 => 1,
        2 => 1_000_000_000,
        _ => panic!("invalid input"),
    };

    String::from_utf8(jump_with_cycle(str.as_bytes().to_vec(), total, &cmds)).unwrap()
}

pub fn run(input: &str) {
    println!("{}", after_dance(input, "abcdefghijklmnop", 1));
    println!("{}", after_dance(input, "abcdefghijklmnop", 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(after_dance("s1,x3/4,pe/b", "abcde", 1), "baedc");
    }
}
