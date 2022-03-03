use std::io;

fn handle_input() -> (f64, f64, f64) {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let str_vec: Vec<&str> = string
        .trim()
        .split(" ")
        .collect();

    let num_vec: Vec<f64> = str_vec.into_iter()
        .map(|str_num| str_num.parse().unwrap()).collect();

    (num_vec[0], num_vec[1], num_vec[2])
}

fn triangle_area(base: f64, height: f64) -> f64 {
    base * height / 2.0
}

fn circle_area(radius: f64) -> f64 {
    3.14159 * radius * radius
}

fn trapezium_area(b0: f64, b1: f64, height: f64) -> f64 {
    (b0 + b1) * height / 2.0
}

fn rectangle_area(base: f64, height: f64) -> f64 {
    base * height
}

fn main() {
    let (a, b, c) = handle_input();

    println!("TRIANGULO: {:.3}", triangle_area(a, c));
    println!("CIRCULO: {:.3}", circle_area(c));
    println!("TRAPEZIO: {:.3}", trapezium_area(a, b, c));
    println!("QUADRADO: {:.3}", rectangle_area(b, b));
    println!("RETANGULO: {:.3}", rectangle_area(a, b));
}
