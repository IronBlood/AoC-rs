use std::collections::VecDeque;

pub enum Oprand {
    REG(usize),
    VAL(i64),
}

impl Oprand {
    pub fn parse(s: &str) -> Self {
        if s.len() == 1 {
            let b = s.as_bytes()[0];
            if b.is_ascii_lowercase() {
                return Oprand::REG((b - b'a') as usize);
            }
        }
        Oprand::VAL(s.parse().expect("not a valid number"))
    }
}

enum Instruction {
    SND(Oprand),
    SET(usize, Oprand),
    ADD(usize, Oprand),
    MUL(usize, Oprand),
    MOD(usize, Oprand),
    RCV(Oprand),
    JGZ(Oprand, Oprand),
}

impl Instruction {
    fn expect_reg(s: &str) -> usize {
        match Oprand::parse(s) {
            Oprand::REG(idx) => idx,
            _ => panic!("lhs must be a register {s}"),
        }
    }

    pub fn parse(data: &str) -> Vec<Instruction> {
        data.lines()
            .map(|line| {
                let ins: Vec<_> = line.split_whitespace().collect();
                match ins[0] {
                    "snd" => {
                        assert!(ins.len() == 2);
                        Instruction::SND(Oprand::parse(ins[1]))
                    }
                    "set" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::SET(op1, op2)
                    }
                    "add" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::ADD(op1, op2)
                    }
                    "mul" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::MUL(op1, op2)
                    }
                    "mod" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::MOD(op1, op2)
                    }
                    "rcv" => {
                        assert!(ins.len() == 2);
                        Instruction::RCV(Oprand::parse(ins[1]))
                    }
                    "jgz" => {
                        assert!(ins.len() == 3);
                        let op1 = Oprand::parse(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::JGZ(op1, op2)
                    }
                    _ => panic!("invalid"),
                }
            })
            .collect()
    }
}

struct State {
    register: [i64; 26],
    instructions: Vec<Instruction>,
    pc: usize,
}

impl State {
    pub fn new(data: &str) -> Self {
        let instructions = Instruction::parse(data);
        State {
            register: [0; 26],
            instructions,
            pc: 0,
        }
    }

    fn expect_valid_idx(idx: usize) {
        assert!(idx < 26, "expect valid idx");
    }

    fn get_val(self: &Self, oprand: &Oprand) -> i64 {
        match oprand {
            Oprand::REG(x) => {
                Self::expect_valid_idx(*x);
                self.register[*x]
            }
            Oprand::VAL(x) => *x,
        }
    }

    fn set_val(self: &mut Self, idx: usize, val: i64) {
        Self::expect_valid_idx(idx);
        self.register[idx] = val;
    }
}

fn exec(data: &str) -> i64 {
    let mut state = State::new(data);
    let mut play = -1;

    while state.pc < state.instructions.len() {
        let mut should_jump = false;
        let mut should_break = false;
        let ins = &state.instructions[state.pc];
        match ins {
            Instruction::SND(x) => play = state.get_val(x),
            Instruction::SET(i, x) => state.set_val(*i, state.get_val(x)),
            Instruction::ADD(i, x) => state.set_val(*i, state.register[*i] + state.get_val(x)),
            Instruction::MUL(i, x) => state.set_val(*i, state.register[*i] * state.get_val(x)),
            Instruction::MOD(i, x) => state.set_val(*i, state.register[*i] % state.get_val(x)),
            Instruction::RCV(x) => {
                if state.get_val(x) != 0 {
                    should_break = true;
                }
            }
            Instruction::JGZ(a, b) => {
                let val = state.get_val(b);
                if state.get_val(a) > 0 {
                    let pc = state.pc as i64 + val;
                    assert!(pc >= 0);
                    state.pc = pc as usize;
                    should_jump = true;
                }
            }
        }

        if should_break {
            break;
        }
        if !should_jump {
            state.pc += 1;
        }
    }

    play
}

struct Program {
    states: [State; 2],
    queues: [VecDeque<i64>; 2],
    counter: [usize; 2],
}

impl Program {
    fn new(data: &str) -> Self {
        let mut states = [State::new(data), State::new(data)];
        states[1].register[15] = 1;
        Program {
            states,
            queues: [VecDeque::new(), VecDeque::new()],
            counter: [0, 0],
        }
    }

    fn exec(self: &mut Self, idx: usize) -> bool {
        if idx != 0 && idx != 1 {
            panic!("invalid idx");
        }

        let state = &mut self.states[idx];

        if state.pc >= state.instructions.len() {
            return false;
        }

        let mut should_jump = false;
        let ins = &state.instructions[state.pc];
        match ins {
            Instruction::SND(x) => {
                self.counter[idx] += 1;
                self.queues[1 - idx].push_back(state.get_val(x));
            }
            Instruction::SET(i, x) => state.set_val(*i, state.get_val(x)),
            Instruction::ADD(i, x) => state.set_val(*i, state.register[*i] + state.get_val(x)),
            Instruction::MUL(i, x) => state.set_val(*i, state.register[*i] * state.get_val(x)),
            Instruction::MOD(i, x) => state.set_val(*i, state.register[*i] % state.get_val(x)),
            Instruction::RCV(x) => {
                if let Some(v) = self.queues[idx].pop_front() {
                    let i = match x {
                        Oprand::REG(i) => i,
                        _ => panic!("expect reg"),
                    };
                    state.set_val(*i, v);
                } else {
                    return false;
                }
            }
            Instruction::JGZ(a, b) => {
                let val = state.get_val(b);
                if state.get_val(a) > 0 {
                    let pc = state.pc as i64 + val;
                    assert!(pc >= 0);
                    state.pc = pc as usize;
                    should_jump = true;
                }
            }
        }

        if !should_jump {
            state.pc += 1;
        }

        true
    }
}

fn exec2(data: &str) -> usize {
    let mut program = Program::new(data);

    loop {
        let ran0 = program.exec(0);
        let ran1 = program.exec(1);
        if !ran0 && !ran1 {
            break;
        }
    }

    program.counter[1]
}

pub fn run(input: &str) {
    println!("{}", exec(input));
    println!("{}", exec2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            exec(
                "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            exec2(
                "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"
            ),
            3
        );
    }
}
