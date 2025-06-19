use crate::days::utils;
use std::collections::{HashMap, HashSet};

pub fn part1() -> String {
    let input = utils::read_input(3);
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();

    let grid = walk_the_grid(first_line);
    let mut hashset = HashSet::new();
    grid.keys().for_each(|key| {
        hashset.insert(key);
    });
    let grid2 = walk_the_grid(second_line);
    let mut hashset2 = HashSet::new();
    grid2.keys().for_each(|key| {
        hashset2.insert(key);
    });
    let mut result = i32::MAX;
    for g in hashset.intersection(&hashset2) {
        let mut innerresult = 0;
        for num in g.split(",") {
            innerresult += num.parse::<i32>().unwrap().abs();
        }
        if innerresult < result {
            result = innerresult;
        }
    }

    result.to_string()
}

pub fn part2() -> String {
    let input = utils::read_input(3);
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();

    let grid = walk_the_grid(first_line);
    let mut hashset = HashSet::new();
    grid.keys().for_each(|key| {
        hashset.insert(key);
    });
    let grid2 = walk_the_grid(second_line);
    let mut hashset2 = HashSet::new();
    grid2.keys().for_each(|key| {
        hashset2.insert(key);
    });
    let mut result = u32::MAX;
    for g in hashset.intersection(&hashset2) {
        let x =
            grid2.get(g.to_string().as_str()).unwrap() + grid.get(g.to_string().as_str()).unwrap();
        if x < result {
            result = x;
        }
    }

    result.to_string()
}

fn walk_the_grid(line: &str) -> HashMap<String, u32> {
    let mut grid = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut distance = 0;
    for drive in line.split(",") {
        let steps = drive[1..].parse::<u32>().unwrap();
        match drive.chars().next().unwrap().to_string().as_str() {
            "R" => {
                for _ in 0..steps {
                    distance += 1;
                    x += 1;
                    let key = format!("{x},{y}");
                    // println!("{}", key);
                    grid.insert(key, distance);
                }
            }

            "U" => {
                for _ in 0..steps {
                    distance += 1;
                    y += 1;
                    let key = format!("{x},{y}");
                    //  println!("{}", key);
                    grid.insert(key, distance);
                }
            }
            "L" => {
                for _ in 0..steps {
                    distance += 1;
                    x -= 1;
                    let key = format!("{x},{y}");
                    //    println!("{}", key);
                    grid.insert(key, distance);
                }
            }
            "D" => {
                for _ in 0..steps {
                    distance += 1;
                    y -= 1;
                    let key = format!("{x},{y}");
                    //     println!("{}", key);
                    grid.insert(key, distance);
                }
            }
            _ => panic!("Should not be possible"),
        }
    }
    grid
}
