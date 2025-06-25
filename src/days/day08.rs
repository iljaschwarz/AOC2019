use crate::days::utils;
use std::collections::HashMap;

pub fn part1() -> String {
    let input = utils::read_input(8);
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let mut cur0 = 999999999;
    let mut cur_result = 0;
    for x in 0..input.len() / layer_size {
        let substr = &input[x * layer_size..(x + 1) * layer_size];
        let (count_0, count_1, count_2) =
            substr.chars().fold((0, 0, 0), |(c0, c1, c2), c| match c {
                '0' => (c0 + 1, c1, c2),
                '1' => (c0, c1 + 1, c2),
                '2' => (c0, c1, c2 + 1),
                _ => (c0, c1, c2),
            });
        if cur0 > count_0 {
            cur0 = count_0;
            cur_result = count_1 * count_2;
        }
    }
    cur_result.to_string()
}
pub fn part2() -> String {
    let input = utils::read_input(8);
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let mut map = HashMap::new();
    for x in 0..input.len() / layer_size {
        let substr = &input[x * layer_size..(x + 1) * layer_size];
        for (index, ch) in substr.chars().enumerate() {
            if map.contains_key(&index) {
                continue;
            }
            match ch {
                '0' => {
                    map.insert(index, ' ');
                }
                '1' => {
                    map.insert(index, '█');
                }
                _ => continue,
            }
        }
    }
    let mut result = String::new();
    for pos in 0..layer_size {
        if pos % width == 0 {
            result.push('\n');
        }
        result.push(*map.get(&pos).unwrap());
    }
    result
}
