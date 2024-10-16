use std::collections::{HashMap, HashSet};

use crate::days::utils;

pub fn part1() -> String {
    let input = utils::read_input(4);
    let mut input = input.split("-");

    let mut result = 0;

    let low = input.next().unwrap().parse::<u32>().unwrap();
    let high = input.next().unwrap().parse::<u32>().unwrap();

    for x in low..high {
        let mut digits: Vec<u32> = x
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        digits.sort();
        let sorted_digitis = digits.iter().map(|d| d.to_string()).collect::<String>();
        if sorted_digitis != x.to_string() {
            continue;
        }
        let set: HashSet<u32> = HashSet::from_iter(digits.clone());
        if set.len() == digits.len() {
            continue;
        }
        result += 1;
    }

    return result.to_string();
}

pub fn part2() -> String {
    let input = utils::read_input(4);
    let mut input = input.split("-");

    let mut result = 0;

    let low = input.next().unwrap().parse::<u32>().unwrap();
    let high = input.next().unwrap().parse::<u32>().unwrap();

    for x in low..high {
        let mut digits: Vec<u32> = x
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        digits.sort();
        let sorted_digitis = digits.iter().map(|d| d.to_string()).collect::<String>();
        if sorted_digitis != x.to_string() {
            continue;
        }
        let mut map: HashMap<u32, u32> = HashMap::new();
        digits.clone().iter().for_each(|num| match map.get(num) {
            Some(x) => {
                map.insert(*num, x + 1);
            }

            None => {
                map.insert(*num, 1);
            }
        });

        match map.values().find(|&&x| x == 2) {
            None => continue,
            Some(_) => {}
        }
        let set: HashSet<u32> = HashSet::from_iter(digits.clone().into_iter());
        if set.len() == digits.len() {
            continue;
        }
        result += 1;
        //  println!("{}", sorted_digitis);
    }

    return result.to_string();
}
