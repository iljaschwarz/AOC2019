use crate::days::utils;

pub fn part1() -> String {
    let input = utils::read_input(1);
    let mut result = 0;
    input.lines().for_each(|line| {
        let num = line.parse::<u32>().unwrap() / 3 - 2;
        result += num;
    });
    return format!("{}", result);
}
pub fn part2() -> String {
    let input = utils::read_input(1);
    let mut result = 0;
    input.lines().for_each(|line| {
        let mut inner_result = 0;
        let mut num = line.parse::<i32>().unwrap() / 3 - 2;
        while num > 0 {
            inner_result += num;
            num = num / 3 - 2;
        }
        result += inner_result;
    });
    return format!("{}", result);
}
