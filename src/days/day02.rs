use crate::days::utils;

pub fn part1() -> String {
    let input = utils::read_input(2);
    let mut intcode = input
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    intcode[1] = 12;
    intcode[2] = 2;
    let mut pos = 0;
    while intcode[pos] != 99 {
        let opcode = intcode[pos];
        let from1 = intcode[pos + 1] as usize;
        let from2 = intcode[pos + 2] as usize;
        let to = intcode[pos + 3] as usize;

        match opcode {
            1 => intcode[to] = intcode[from1] + intcode[from2],

            2 => intcode[to] = intcode[from1] * intcode[from2],

            _ => panic!("Should not happen"),
        }
        pos += 4
    }
    return intcode[0].to_string();
}

pub fn part2() -> String {
    let mut result = 0;
    let input = utils::read_input(2);
    for x in 0..100 {
        for y in 0..100 {
            let mut intcode = input
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            intcode[1] = x;
            intcode[2] = y;
            let mut pos = 0;
            while intcode[pos] != 99 {
                let opcode = intcode[pos];
                let from1 = intcode[pos + 1] as usize;
                let from2 = intcode[pos + 2] as usize;
                let to = intcode[pos + 3] as usize;

                match opcode {
                    1 => intcode[to] = intcode[from1] + intcode[from2],

                    2 => intcode[to] = intcode[from1] * intcode[from2],

                    _ => panic!("Should not happen"),
                }
                pos += 4
            }
            if intcode[0] == 19690720 {
                result = 100 * x + y;
                break;
            }
        }
    }
    return result.to_string();
}
