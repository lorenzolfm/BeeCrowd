use std::io;

fn handle_input() -> f64 {
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .unwrap();

    string.trim().parse::<f64>().unwrap()
}

fn consumption(total_distance: i32, total_spent_fuel: f64) -> f64 {
    total_distance as f64 / total_spent_fuel
}

fn main() {
    let total_distance = handle_input() as i32;
    let total_spent_fuel = handle_input();

    println!("{:.3} km/l", consumption(total_distance, total_spent_fuel));
}
