use crate::computer::Computer;
use crate::days::utils;

pub fn part1() -> String {
    let input = utils::read_input(5);
    let mut comp = Computer::new(input, vec![1]);

    comp.run_until_halted().last().unwrap().to_string()
    
}

pub fn part2() -> String {
    let input = utils::read_input(5);
    let mut comp = Computer::new(input, vec![5]);

    comp.run_until_halted().last().unwrap().to_string()
}
