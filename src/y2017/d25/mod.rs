use std::collections::HashMap;

use regex::Regex;

struct Condition {
    pub next_value: u8,
    pub next_move: i8,
    pub next_state: u8,
}

struct Tape {
    buffer: Vec<u8>,
    idx: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            buffer: vec![0, 0, 0],
            idx: 0,
        }
    }

    fn enlarge(&mut self) {
        let len = self.buffer.len();
        let mut bigger = vec![0; len * 3];
        bigger[len..len * 2].copy_from_slice(&self.buffer);
        self.buffer = bigger;
        self.idx += len;
    }

    pub fn do_move(&mut self, dir: i8) {
        assert!(
            dir == 1 || dir == -1,
            "direction must be +1 or -1, got {}",
            dir
        );
        if (self.idx == 0 && dir == -1) || (self.idx == self.buffer.len() - 1 && dir == 1) {
            self.enlarge();
        }
        if dir == -1 {
            self.idx -= 1;
        } else {
            self.idx += 1;
        }
    }

    pub fn set_val(&mut self, val: u8) {
        assert!(val == 0 || val == 1, "val must be 0 or 1, got {}", val);
        self.buffer[self.idx] = val;
    }

    pub fn get_val(&self) -> u8 {
        self.buffer[self.idx]
    }

    // replacing loop through, suggested by ChatGPT
    pub fn checksum(&self) -> u32 {
        self.buffer
            .iter()
            .fold(0u32, |acc, &bit| acc + u32::from(bit))
    }
}

fn extract_char(line: &str) -> u8 {
    line.bytes().rev().nth(1).unwrap()
}

fn checksum(data: &str) -> u32 {
    let mut parts = data.split("\n\n");
    let init = parts.next().unwrap();

    let states_str = parts;
    let tmp: Vec<&str> = init.split("\n").collect();

    assert!(tmp.len() == 2, "invalid data");
    let re = Regex::new(r"\d+").unwrap();
    let iterations: Vec<&str> = re.find_iter(tmp[1]).map(|m| m.as_str()).collect();
    assert!(iterations.len() == 1, "invalid data");
    let mut iterations: i32 = iterations[0].parse::<i32>().expect("not a valid i32");

    let tmp: Vec<&str> = tmp[0].split(" ").collect();
    let begin_state = tmp[tmp.len() - 1].as_bytes()[0];
    let mut map: HashMap<u8, Vec<Condition>> = HashMap::new();

    states_str.for_each(|str| {
        let lines: Vec<&str> = str.lines().collect();
        assert!(lines.len() == 9, "invalid input");
        let id = extract_char(lines[0]);

        let str_conditions = vec![&lines[2..5], &lines[6..]];

        let conditions = str_conditions
            .iter()
            .map(|cfg| {
                assert!(cfg.len() == 3, "invalid cfg");
                let next_value = extract_char(cfg[0]) - b'0';
                let next_move = match cfg[1].ends_with("right.") {
                    true => 1,
                    false => -1,
                };
                let next_state = extract_char(cfg[2]);

                Condition {
                    next_value,
                    next_move,
                    next_state,
                }
            })
            .collect();
        map.insert(id, conditions);
    });

    let mut tape = Tape::new();
    let mut curr_state = begin_state;

    while iterations > 0 {
        iterations -= 1;

        let conditions = map.get(&curr_state).expect("state not found");
        let condition = conditions.get(tape.get_val() as usize).unwrap();

        tape.set_val(condition.next_value);
        tape.do_move(condition.next_move);
        curr_state = condition.next_state;
    }

    tape.checksum()
}

pub fn run(input: &str) {
    println!("{}", checksum(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
        "
        .trim();
        assert_eq!(checksum(s), 3);
    }
}
