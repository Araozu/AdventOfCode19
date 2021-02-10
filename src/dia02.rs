use std::time::Instant;
use std::convert::TryFrom;
use crate::utils;

pub fn run() {
    let time = Instant::now();
    let data = utils::get_data_as_str(2, false);

    let mut vec = data.split(",").map(|e| {
        e.parse::<i32>().unwrap()
    }).collect::<Vec<_>>();

    let mut i = 0;
    let vec_size = vec.len();
    while i < vec_size {
        match vec[i] {
            1 => {
                let first_position = usize::try_from(vec[i + 1]).unwrap();
                let second_position = usize::try_from(vec[i + 2]).unwrap();
                let third_position = usize::try_from(vec[i + 3]).unwrap();

                let first_value = vec[first_position];
                let second_value = vec[second_position];
                let result = first_value + second_value;

                vec[third_position] = result;
                i += 4;
            }
            2 => {
                let first_position = usize::try_from(vec[i + 1]).unwrap();
                let second_position = usize::try_from(vec[i + 2]).unwrap();
                let third_position = usize::try_from(vec[i + 3]).unwrap();

                let first_value = vec[first_position];
                let second_value = vec[second_position];
                let result = first_value * second_value;

                vec[third_position] = result;
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

    println!("Test: {}", vec[0]);
    println!("\tMicroseconds: {:?}", time.elapsed().as_micros());
}
