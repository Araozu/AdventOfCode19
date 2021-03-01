use std::time::Instant;
use crate::utils;
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

struct Rect {
    start: Point,
    end: Point,
    orientation: Orientation
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let manhattan_1 = self.x.abs() + self.y.abs();
        let manhattan_2 = other.x.abs() + other.y.abs();
        manhattan_1.cmp(&manhattan_2)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_point(instruction: &str, previous_point: Point) -> (Point, Orientation) {
    let direction = &instruction[0..1].chars().nth(0).unwrap();
    let value = &instruction[1..].parse::<i32>().unwrap();

    match direction {
        'R' => {
            let next_point = Point {
                x: previous_point.x + value,
                y: previous_point.y
            };
            (next_point, Orientation::Horizontal)
        }
        'L' => {
            let next_point = Point {
                x: previous_point.x - value,
                y: previous_point.y
            };
            (next_point, Orientation::Horizontal)
        }
        'U' => {
            let next_point = Point {
                x: previous_point.x,
                y: previous_point.y + value
            };
            (next_point, Orientation::Vertical)
        }
        'D' => {
            let next_point = Point {
                x: previous_point.x,
                y: previous_point.y - value
            };
            (next_point, Orientation::Vertical)
        }
        _ => {
            panic!("Invalid instruction in day 3")
        }
    }
}

fn parse_map(data: &String, starting_point: Point) -> (Vec<Rect>, Vec<Rect>) {
    let mut previous_point = starting_point;

    let mut horizontal_rects = Vec::<Rect>::new();
    let mut vertical_rects = Vec::<Rect>::new();

    data.split(",").for_each(|x| {
        let (next_point, orientation) = parse_point(x, previous_point);
        let return_value = Rect {
            start: previous_point,
            end: next_point,
            orientation
        };
        previous_point = next_point;

        match orientation {
            Orientation::Horizontal => {
                horizontal_rects.push(return_value)
            }
            Orientation::Vertical => {
                vertical_rects.push(return_value)
            }
        }
    });

    (horizontal_rects, vertical_rects)
}

// Assumes one rect is vertical and the other is horizontal
fn rect_intersection(rect_1: &Rect, rect_2: &Rect) -> Option<Point> {

    let (vertical, horizontal) =
        match rect_1.orientation {
            Orientation::Vertical => (rect_1, rect_2),
            Orientation::Horizontal => (rect_2, rect_1)
        };

    // h.x  v.x  h.x  &&  v.y  h.y  v.y
    let (horizontal_x_min, horizontal_x_max) = {
        if horizontal.end.x > horizontal.start.x {
            (horizontal.start.x, horizontal.end.x)
        } else {
            (horizontal.end.x, horizontal.start.x)
        }
    };
    let vertical_x = vertical.start.x;

    let (vertical_y_min, vertical_y_max) = {
        if vertical.end.y > vertical.start.y {
            (vertical.start.y, vertical.end.y)
        } else {
            (vertical.end.y, vertical.start.y)
        }
    };
    let horizontal_y = horizontal.start.y;

    let rects_intersect = (horizontal_x_min <= vertical_x && vertical_x <= horizontal_x_max)
        && (vertical_y_min <= horizontal_y && horizontal_y <= vertical_y_max);

    if rects_intersect {
        Option::Some(Point {
            x: vertical_x,
            y: horizontal_y
        })
    } else {
        Option::None
    }
}

// Assumes one vector contains only vertical rects, and the other only horizontal rects
fn find_crossing_points(first_path: Vec<Rect>, second_path: Vec<Rect>) -> Vec<Point> {
    let mut intersection_points = Vec::<Point>::new();

    for r1 in &first_path {
        for r2 in &second_path {
            match rect_intersection(r1, r2) {
                Option::Some(point) => {
                    // Prevent the origin (0, 0) from counting
                    if !(point.x == 0 && point.y == 0) {
                        intersection_points.push(point)
                    }
                }
                _ => {}
            }
        }
    };

    intersection_points
}

pub fn run() {
    let time = Instant::now();
    let data = utils::get_data_as_str(3, false);

    let paths = data.split("\n").map(|x| String::from(x)).collect::<Vec<_>>();
    let first_path_str = paths.get(0).unwrap();
    let second_path_str = paths.get(1).unwrap();

    let p = Point {x: 0, y: 0};

    let (first_horizontal, first_vertical) = parse_map(first_path_str, p);
    let (second_horizontal, second_vertical) = parse_map(second_path_str, p);

    // Find crossing points
    let crossing_points_1 = find_crossing_points(first_horizontal, second_vertical);
    let crossing_points_2 = find_crossing_points(first_vertical, second_horizontal);

    let min_1 = crossing_points_1.iter().min().unwrap();
    let min_2 = crossing_points_2.iter().min().unwrap();

    let min = if min_1 < min_2 {
        min_1
    } else {
        min_2
    };

    println!("\tPart 3: {}", min.x.abs() + min.y.abs());
    println!("\tMicroseconds: {:?}", time.elapsed().as_micros());
}









