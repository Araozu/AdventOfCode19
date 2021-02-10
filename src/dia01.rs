use crate::utils;
use std::time::Instant;

fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn get_recursive_fuel(mass: i32) -> i32 {
    let mass = (mass / 3) - 2;
    if mass <= 0 {
        0
    } else if mass < 6 {
        mass
    } else {
        mass + get_recursive_fuel(mass)
    }
}

pub fn run() {
    let start = Instant::now();
    let data = utils::get_data_as_str(1, false);
    let data = data.trim();
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    for s in data.split("\n") {
        match s.parse() {
            Ok(n) => {
                total_part_1 += get_fuel(n);
                total_part_2 += get_recursive_fuel(n);
            }
            Err(_) => {
                println!("Error found when parsing {}", s);
                continue;
            }
        }
    }

    println!("Day 1 results:\n\tPart 1: {}\n\tPart 2: {}", total_part_1, total_part_2);
    println!("\tMicroseconds: {:?}", start.elapsed().as_micros());
}
