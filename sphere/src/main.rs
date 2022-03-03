use std::io;

fn handle_input() -> f64 {
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .unwrap();

    string.trim().parse::<f64>().unwrap()
}

fn sphere_volume(radius: f64) -> f64 {
    (4.0 / 3.0) * 3.14159 * radius * radius * radius
}

fn main() {
    let radius = handle_input();
    let volume = sphere_volume(radius);

    println!("VOLUME = {:.3}", volume)
}
