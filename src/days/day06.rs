use crate::days::utils;
use std::collections::HashMap;

pub fn part1() -> String {
    let input = utils::read_input(6);

    let map = generate_map(input);
    let mut count = 0;
    for mut child in map.keys() {
        while let Some(parent) = map.get(child) {
            count += 1;
            child = parent;
        }
    }
    count.to_string()
}

pub fn part2() -> String {
    let input = utils::read_input(6);

    let map = generate_map(input);
    let you = generate_parents(&String::from("YOU"), &map);
    let san = generate_parents(&String::from("SAN"), &map);

    let mut result = 0;
    if let Some((pos, _)) = you.iter().enumerate().find_map(|(pos, parent)| {
        san.iter().enumerate().find_map(|(pos2, parent2)| {
            if parent == parent2 {
                Some((pos + pos2, ()))
            } else {
                None
            }
        })
    }) {
        result = pos;
    }

    result.to_string()
}
fn generate_parents(child: &String, map: &HashMap<String, String>) -> Vec<String> {
    let mut parents: Vec<String> = Vec::new();
    let mut child = child;
    while let Some(parent) = map.get(child) {
        parents.push(parent.to_string());
        child = parent;
    }
    parents
}
fn generate_map(input: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        let split = line.split(")").collect::<Vec<&str>>();
        let parent = split[0].to_string();
        let child = split[1].to_string();
        map.insert(child, parent);
    });
    map
}
