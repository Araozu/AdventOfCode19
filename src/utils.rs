use std::fs;

const PATH: &str = "/home/araozu/Programacion/Advent/2019/";

fn get_day_string(day: u8) -> String {
    let day_s = day.to_string();
    if day_s.len() == 1 {
        format!("0{}", day_s)
    } else {
        day_s
    }
}

pub fn get_data_as_str(day: u8, is_test: bool) -> String {
    let day_str = get_day_string(day) + if is_test { "_test" } else { "" };
    let path = format!("{}{}", PATH, day_str);
    fs::read_to_string(path).unwrap()
}


