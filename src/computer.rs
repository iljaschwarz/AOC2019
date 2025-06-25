#! [allow(unused)]

#[derive(Debug)]
pub struct Computer {
    intcode: Vec<i32>,
    input_id: Vec<i32>,
    pos: usize,
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
                return result
            }
            result.push(state.get_result());
        }
    }
    pub fn compute(&mut self) -> ComputerState {
        if(self.halted) {
            return ComputerState {
                result: "HALTED".to_string(),
                halted: true,
            };
        }
        while self.intcode[self.pos] != 99 {
            let opcode = self.intcode[self.pos];
            let a = opcode % 10;
            //let b = opcode / 10 % 10;
            let c = (opcode / 100 % 10) == 1;
            let d = (opcode / 1000 % 10) == 1;
            //let e = (opcode / 10000 % 10) == 1;
            match a {
                1 => {
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
                    self.intcode[to] = op1 + op2;
                    self.pos += 4
                }

                2 => {
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
                    self.intcode[to] = op1 * op2;
                    self.pos += 4
                }

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

                _ => panic!("Should not happen"),
            }
        }
        self.halted = true;
        ComputerState {
            result: "HALTED".to_string(),
            halted: true,
        }
    }
}

pub fn get_intcode_from_str(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
