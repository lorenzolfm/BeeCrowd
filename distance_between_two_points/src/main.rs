use std::io;

fn handle_input() -> Point {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let str_vec: Vec<&str> = string
        .trim()
        .split(" ")
        .collect();

    let num_vec: Vec<f64> = str_vec.into_iter()
        .map(|str_num| str_num.parse().unwrap()).collect();

    Point { x: num_vec[0], y: num_vec[1] }
}

struct Point {
    x: f64,
    y: f64,
}

fn distance_between_two_points(p0: Point, p1: Point) -> f64 {
    ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt()
}

fn main() {
    let p0 = handle_input();
    let p1 = handle_input();

    let distance = distance_between_two_points(p0, p1);
    println!("{:.4}", distance);
}
