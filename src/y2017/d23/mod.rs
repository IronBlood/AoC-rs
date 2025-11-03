use super::d18::Oprand;

enum Instruction {
    SET(usize, Oprand),
    SUB(usize, Oprand),
    MUL(usize, Oprand),
    JNZ(Oprand, Oprand),
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
                    "set" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::SET(op1, op2)
                    }
                    "sub" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::SUB(op1, op2)
                    }
                    "mul" => {
                        assert!(ins.len() == 3);
                        let op1 = Self::expect_reg(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::MUL(op1, op2)
                    }
                    "jnz" => {
                        assert!(ins.len() == 3);
                        let op1 = Oprand::parse(ins[1]);
                        let op2 = Oprand::parse(ins[2]);
                        Instruction::JNZ(op1, op2)
                    }
                    _ => panic!("invalid"),
                }
            })
            .collect()
    }
}

struct State {
    register: [i64; 8],
    instructions: Vec<Instruction>,
    pc: usize,
}

impl State {
    pub fn new(data: &str) -> Self {
        let instructions = Instruction::parse(data);
        State {
            register: [0; 8],
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

fn exec(data: &str) -> u32 {
    let mut state = State::new(data);
    let mut count_mul = 0;

    while state.pc < state.instructions.len() {
        let mut should_jump = false;
        let ins = &state.instructions[state.pc];
        match ins {
            Instruction::SET(i, x) => state.set_val(*i, state.get_val(x)),
            Instruction::SUB(i, x) => state.set_val(*i, state.register[*i] - state.get_val(x)),
            Instruction::MUL(i, x) => {
                state.set_val(*i, state.register[*i] * state.get_val(x));
                count_mul += 1;
            }
            Instruction::JNZ(a, b) => {
                if state.get_val(a) != 0 {
                    let pc = state.pc as i64 + state.get_val(b);
                    assert!(pc >= 0);
                    state.pc = pc as usize;
                    should_jump = true;
                }
            }
        }
        if !should_jump {
            state.pc += 1;
        }
    }

    count_mul
}

fn exec2(data: &str) -> u32 {
    let mut count = 0;
    let mut b = data
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse::<u32>()
        .unwrap()
        * 100
        + 100000;
    let c = b + 17000;
    while b <= c {
        let mut i = 2;
        while i * i <= b {
            if b % i == 0 {
                count += 1;
                break;
            }
            i += 1;
        }
        b += 17;
    }

    count
}

pub fn run(input: &str) {
    println!("{}", exec(input));
    println!("{}", exec2(input));
}
