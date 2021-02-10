use std::time::Instant;
use std::convert::TryFrom;
use crate::utils;

fn execute_order_1(vec: &mut Vec<i32>, i: usize) {
    let first_position = usize::try_from(vec[i + 1]).unwrap();
    let second_position = usize::try_from(vec[i + 2]).unwrap();
    let third_position = usize::try_from(vec[i + 3]).unwrap();

    let first_value = vec[first_position];
    let second_value = vec[second_position];
    let result = first_value + second_value;

    vec[third_position] = result;
}

fn execute_order_2(vec: &mut Vec<i32>, i: usize) {
    let first_position = usize::try_from(vec[i + 1]).unwrap();
    let second_position = usize::try_from(vec[i + 2]).unwrap();
    let third_position = usize::try_from(vec[i + 3]).unwrap();

    let first_value = vec[first_position];
    let second_value = vec[second_position];
    let result = first_value * second_value;

    vec[third_position] = result;
}

fn run_with_values(vec: &mut Vec<i32>, first: i32, second: i32) -> i32 {
    vec[1] = first;
    vec[2] = second;

    let mut i = 0;
    let vec_size = vec.len();
    while i < vec_size {
        match vec[i] {
            1 => {
                execute_order_1(vec, i);
                i += 4;
            }
            2 => {
                execute_order_2(vec, i);
                i += 4;
            }
            99 => {
                break;
            }
            _ => {
                panic!("Day 2: Invalid code");
            }
        }
    }

    vec[0]
}

pub fn run() {
    let time = Instant::now();
    let data = utils::get_data_as_str(2, false);

    let original_vec = data.split(",").map(|e| {
        e.parse::<i32>().unwrap()
    }).collect::<Vec<_>>();

    let mut vec = original_vec.clone();

    println!("Day 2 results:");
    println!("\tPart 1: {}", run_with_values(&mut vec, 12, 2));

    let mut i = 0;
    let mut j = 0;
    while i < 100 {
        j = 0;

        let mut flag = false;
        while j < 100 {
            let mut vec = original_vec.clone();
            let res = run_with_values(&mut vec, i, j);
            if res == 19690720 {
                flag = true;
                break;
            }

            j += 1;
        }
        if flag {
            break;
        }

        i += 1;
    }

    println!("\tPart 2: {}", 100 * i + j);
    println!("\tMicroseconds: {:?}", time.elapsed().as_micros());
}
