use std::fs;

const PATH: &str = "/home/araozu/Programacion/Rust/advent_19/inputs/";

fn get_day_string(day: u8) -> String {
    let day_s = day.to_string();
    if day_s.len() == 1 {
        format!("0{}", day_s)
    } else {
        day_s
    }
}

fn get_data(day: u8) -> String {
    let path = format!("{}{}", PATH, get_day_string(day));
    fs::read_to_string(path).expect("D:")
}

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

fn main() {

    let data = get_data(1);
    let data = data.trim();
    let mut total = 0;
    for s in data.split("\n") {
        match s.parse() {
            Ok(n) => total += get_recursive_fuel(n),
            Err(_) => {
                println!("Se encontro un error al parsear el valor {}", s);
                continue
            }
        }
    }
    println!("Total: {}", total);

}
