use crate::computer::{Computer, ComputerState};
use crate::days::utils;
use itertools::Itertools;

pub fn part1() -> String {
    let input = utils::read_input(7);
    let mut result = 0;
    let permutations = vec![0, 1, 2, 3, 4];
    for perm in permutations
        .iter()
        .permutations(permutations.len())
        .unique()
    {
        let mut curr_result = 0;
        for x in perm {
            let mut comp = Computer::new(input.clone(), vec![curr_result, x.clone()]);
            let state = comp.compute();
            curr_result = state.get_result().parse().unwrap();
        }
        if result < curr_result {
            result = curr_result;
        }
    }

    result.to_string()
}

pub fn part2() -> String {
    let permutations = vec![9, 8, 7, 6, 5];
    let input = utils::read_input(7);
    let mut max = 0;

    for perm in permutations
        .iter()
        .permutations(permutations.len())
        .unique()
    {
        let mut comps = vec![];
        let mut curr_result = 0;
        for x in perm {
            let mut comp = Computer::new(input.clone(), vec![curr_result, x.clone()]);
            curr_result = comp.compute().get_result().parse().unwrap();
            comps.push(comp);
        }
        loop {
            let mut result = ComputerState {
                result: "".to_string(),
                halted: false,
            };
            for comp in comps.iter_mut() {
                comp.push_input(curr_result);
                result = comp.compute();
                if result.is_halted() {
                    break;
                }
                curr_result = result.get_result().parse().unwrap();
            }
            if result.is_halted() {
                break;
            }
        }
        if max < curr_result {
            max = curr_result;
        }
    }
    max.to_string()
}
