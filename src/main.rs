mod computer;
mod days;

use std::env;

use days::{day01, day02, day03, day04, day05, day06, day07, day08};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let result = get_day(day, part)();
    println!("{}", result);
}

fn get_day(day: &str, part: &str) -> Box<dyn Fn() -> String> {
    match (day.to_owned() + part).as_str() {
        "11" => Box::new(day01::part1),
        "12" => Box::new(day01::part2),
        "21" => Box::new(day02::part1),
        "22" => Box::new(day02::part2),
        "31" => Box::new(day03::part1),
        "32" => Box::new(day03::part2),
        "41" => Box::new(day04::part1),
        "42" => Box::new(day04::part2),
        "51" => Box::new(day05::part1),
        "52" => Box::new(day05::part2),
        "61" => Box::new(day06::part1),
        "62" => Box::new(day06::part2),
        "71" => Box::new(day07::part1),
        "72" => Box::new(day07::part2),
        "81" => Box::new(day08::part1),
        "82" => Box::new(day08::part2),
        "91" => panic!("Day not found"),
        "92" => panic!("Day not found"),
        "101" => panic!("Day not found"),
        "102" => panic!("Day not found"),
        "111" => panic!("Day not found"),
        "112" => panic!("Day not found"),
        "121" => panic!("Day not found"),
        "122" => panic!("Day not found"),
        "131" => panic!("Day not found"),
        "132" => panic!("Day not found"),
        "141" => panic!("Day not found"),
        "142" => panic!("Day not found"),
        "151" => panic!("Day not found"),
        "152" => panic!("Day not found"),
        "161" => panic!("Day not found"),
        "162" => panic!("Day not found"),
        "171" => panic!("Day not found"),
        "172" => panic!("Day not found"),
        "181" => panic!("Day not found"),
        "182" => panic!("Day not found"),
        "191" => panic!("Day not found"),
        "192" => panic!("Day not found"),
        "201" => panic!("Day not found"),
        "202" => panic!("Day not found"),
        "211" => panic!("Day not found"),
        "212" => panic!("Day not found"),
        "221" => panic!("Day not found"),
        "222" => panic!("Day not found"),
        "231" => panic!("Day not found"),
        "232" => panic!("Day not found"),
        "241" => panic!("Day not found"),
        "242" => panic!("Day not found"),
        "251" => panic!("Day not found"),
        "252" => panic!("Day not found"),
        _ => panic!("Day not found"),
    }
}
