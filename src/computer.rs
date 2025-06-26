#![allow(unused)]

#[derive(Debug)]
pub struct Computer {
    intcode: Vec<i32>,
    input_id: Vec<i32>,
    pos: usize,
    relative_base: usize,
    halted: bool,
}
#[derive(Debug)]
pub struct ComputerState {
    pub(crate) result: String,
    pub(crate) halted: bool,
}

impl ComputerState {
    pub fn get_result(&self) -> String {
        self.result.clone()
    }
    pub fn is_halted(&self) -> bool {
        self.halted
    }
}

impl Computer {
    pub fn new(input: String, input_id: Vec<i32>) -> Self {
        Computer {
            intcode: get_intcode_from_str(input),
            input_id,
            pos: 0,
            relative_base: 0,
            halted: false,
        }
    }
    pub fn push_input(&mut self, input: i32) {
        self.input_id.push(input);
    }
    pub fn run_until_halted(&mut self) -> Vec<String> {
        let mut result = vec![];
        loop {
            let state = self.compute();
            if state.is_halted() {
                return result;
            }
            result.push(state.get_result());
        }
    }
    fn get_operand(&mut self, a: Mode, b: Mode) -> (i32, i32) {
        let op1 = match a {
            Mode::Position => self.intcode[self.pos + 1],
            Mode::Position => self.intcode[self.intcode[self.pos + 1] as usize],
            _ => panic!("Not Implemented Mode {:?}", a),
        };
        let op2 = match b {
            Mode::Position => self.intcode[self.pos + 2],
            Mode::Position => self.intcode[self.intcode[self.pos + 2] as usize],
            _ => panic!("Not Implemented Mode {:?}", b),
        };
        (op1, op2)
    }
    fn math(&mut self, a: Mode, b: Mode, op: &str) {
        let (op1, op2) = self.get_operand(a, b);
        let to = self.intcode[self.pos + 3] as usize;
        self.intcode[to] = match op {
            "+" => op1 + op2,
            "*" => op1 * op2,
            _ => panic!("Not Implemented Mode {:?}", op),
        };
        self.pos += 4
    }

    pub fn compute(&mut self) -> ComputerState {
        if (self.halted) {
            return ComputerState {
                result: "HALTED".to_string(),
                halted: true,
            };
        }
        while !self.halted {
            let opcode = get_opcode(self.intcode[self.pos]);

            match opcode {
                OpCode::Add(a, b) => {
                    self.math(a, b, "+");
                }
                OpCode::Multiply(a, b) => {
                    self.math(a, b, "*");
                }
                OpCode::Input=>{},
                OpCode::Output=>{},
                OpCode::JumpIfTrue=>{},
                OpCode::JumpIfFalse=>{},
                OpCode::LessThan=>{},
                OpCode::Equals=>{},
                OpCode::Halt=>{},
                3 => {
                    let to = self.intcode[self.pos + 1] as usize;
                    self.intcode[to] = self.input_id.pop().unwrap();
                    self.pos += 2
                }
                4 => {
                    if c {
                        let return_result = self.intcode[self.pos + 1].to_string();
                        self.pos += 2;
                        return ComputerState {
                            result: return_result,
                            halted: false,
                        };
                    } else {
                        let return_result =
                            self.intcode[self.intcode[self.pos + 1] as usize].to_string();
                        self.pos += 2;
                        return ComputerState {
                            result: return_result,
                            halted: false,
                        };
                    }
                }
                5 => {
                    let op1 = if c {
                        self.intcode[self.pos + 1]
                    } else {
                        self.intcode[self.intcode[self.pos + 1] as usize]
                    };
                    let op2 = if d {
                        self.intcode[self.pos + 2]
                    } else {
                        self.intcode[self.intcode[self.pos + 2] as usize]
                    };

                    if op1 != 0 {
                        self.pos = op2 as usize
                    } else {
                        self.pos += 3
                    }
                }
                6 => {
                    let op1 = if c {
                        self.intcode[self.pos + 1]
                    } else {
                        self.intcode[self.intcode[self.pos + 1] as usize]
                    };
                    let op2 = if d {
                        self.intcode[self.pos + 2]
                    } else {
                        self.intcode[self.intcode[self.pos + 2] as usize]
                    };

                    if op1 == 0 {
                        self.pos = op2 as usize
                    } else {
                        self.pos += 3
                    }
                }
                7 => {
                    let op1 = if c {
                        self.intcode[self.pos + 1]
                    } else {
                        self.intcode[self.intcode[self.pos + 1] as usize]
                    };
                    let op2 = if d {
                        self.intcode[self.pos + 2]
                    } else {
                        self.intcode[self.intcode[self.pos + 2] as usize]
                    };

                    let to = self.intcode[self.pos + 3] as usize;

                    self.intcode[to] = (op1 < op2) as i32;

                    self.pos += 4
                }
                8 => {
                    let op1 = if c {
                        self.intcode[self.pos + 1]
                    } else {
                        self.intcode[self.intcode[self.pos + 1] as usize]
                    };
                    let op2 = if d {
                        self.intcode[self.pos + 2]
                    } else {
                        self.intcode[self.intcode[self.pos + 2] as usize]
                    };

                    let to = self.intcode[self.pos + 3] as usize;

                    self.intcode[to] = (op1 == op2) as i32;

                    self.pos += 4
                }
                99 => {
                    self.halted = true;
                    break;
                }
                _ => panic!("Should not happen"),
            }
        }
        ComputerState {
            result: "HALTED".to_string(),
            halted: true,
        }
    }
}
#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}
enum OpCode {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}
fn get_opcode(opcode: i32) -> OpCode {
    let a = opcode % 100;
    //let b = opcode / 10 % 10;
    let c = (opcode / 100 % 10);
    let d = (opcode / 1000 % 10);
    let e = (opcode / 10000 % 10);

    let mode_a = get_mode(c);
    let mode_b = get_mode(d);
    let _mode_c = get_mode(e);

    match a {
        1 => OpCode::Add(mode_a, mode_b),
        2 => OpCode::Multiply(mode_a, mode_b),
        3 => OpCode::Input,
        4 => OpCode::Output,
        5 => OpCode::JumpIfTrue,
        6 => OpCode::JumpIfFalse,
        7 => OpCode::LessThan,
        8 => OpCode::Equals,
        99 => OpCode::Halt,
        _ => {
            panic!("Should not happen")
        }
    }
}

fn get_mode(c: i32) -> Mode {
    match c {
        1 => Mode::Position,
        2 => Mode::Immediate,
        3 => Mode::Relative,
        _ => panic!(),
    }
}

pub fn get_intcode_from_str(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
