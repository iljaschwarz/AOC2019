use crate::days::utils;

pub fn part1() -> String {
    let input = utils::read_input(5);
    let intcode = input
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let (_, result) = computer(intcode, 1);
    result.last().unwrap().clone()
}

pub fn part2() -> String {
    let input = utils::read_input(5);
    let intcode = input
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let (_, result) = computer(intcode, 5);
    result.last().unwrap().clone()
}
pub fn computer(mut intcode: Vec<i32>, input_id: i32) -> (Vec<i32>, Vec<String>) {
    let mut result = vec![];
    let mut pos = 0;
    while intcode[pos] != 99 {
        let opcode = intcode[pos];
        let a = opcode % 10;
        //let b = opcode / 10 % 10;
        let c = (opcode / 100 % 10) == 1;
        let d = (opcode / 1000 % 10) == 1;
        //let e = (opcode / 10000 % 10) == 1;

        match a {
            1 => {
                let op1 = if c {
                    intcode[pos + 1]
                } else {
                    intcode[intcode[pos + 1] as usize]
                };
                let op2 = if d {
                    intcode[pos + 2]
                } else {
                    intcode[intcode[pos + 2] as usize]
                };

                let to = intcode[pos + 3] as usize;
                intcode[to] = op1 + op2;
                pos += 4
            }

            2 => {
                let op1 = if c {
                    intcode[pos + 1]
                } else {
                    intcode[intcode[pos + 1] as usize]
                };
                let op2 = if d {
                    intcode[pos + 2]
                } else {
                    intcode[intcode[pos + 2] as usize]
                };

                let to = intcode[pos + 3] as usize;
                intcode[to] = op1 * op2;
                pos += 4
            }

            3 => {
                let to = intcode[pos + 1] as usize;
                intcode[to] = input_id;
                pos += 2
            }
            4 => {
                if c {
                    result.push(intcode[pos + 1].to_string());
                } else {
                    result.push(intcode[intcode[pos + 1] as usize].to_string());
                }

                pos += 2
            }
            5 => {
                let op1 = if c {
                    intcode[pos + 1]
                } else {
                    intcode[intcode[pos + 1] as usize]
                };
                let op2 = if d {
                    intcode[pos + 2]
                } else {
                    intcode[intcode[pos + 2] as usize]
                };

                if op1 != 0 {
                    pos = op2 as usize
                } else {
                    pos += 3
                }
            }
            6 => {
                let op1 = if c {
                    intcode[pos + 1]
                } else {
                    intcode[intcode[pos + 1] as usize]
                };
                let op2 = if d {
                    intcode[pos + 2]
                } else {
                    intcode[intcode[pos + 2] as usize]
                };

                if op1 == 0 {
                    pos = op2 as usize
                } else {
                    pos += 3
                }
            }
            7 => {
                let op1 = if c {
                    intcode[pos + 1]
                } else {
                    intcode[intcode[pos + 1] as usize]
                };
                let op2 = if d {
                    intcode[pos + 2]
                } else {
                    intcode[intcode[pos + 2] as usize]
                };

                let to = intcode[pos + 3] as usize;

                intcode[to] = (op1 < op2) as i32;

                pos += 4
            }
            8 => {
                let op1 = if c {
                    intcode[pos + 1]
                } else {
                    intcode[intcode[pos + 1] as usize]
                };
                let op2 = if d {
                    intcode[pos + 2]
                } else {
                    intcode[intcode[pos + 2] as usize]
                };

                let to = intcode[pos + 3] as usize;

                intcode[to] = (op1 == op2) as i32;

                pos += 4
            }

            _ => panic!("Should not happen"),
        }
    }
    (intcode, result)
}
