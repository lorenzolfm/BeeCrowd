use std::io;

fn handle_input() -> i32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn fuel_spent(spent_time_in_trip: i32, avg_speed: i32) -> f64 {
    let consumption = 12;
    spent_time_in_trip as f64 * avg_speed as f64 / consumption as f64
}

fn main() {
    let spent_time_in_trip = handle_input();
    let avg_speed = handle_input();

    println!("{:.3}", fuel_spent(spent_time_in_trip, avg_speed));
}
