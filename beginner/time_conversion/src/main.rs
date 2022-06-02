use std::io;

fn handle_input() -> i32 {
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .unwrap();

    string.trim().parse::<i32>().unwrap()
}

fn time_conversion(duration_in_seconds: i32) {
    let hour = duration_in_seconds / 3600;
    let minutes = (duration_in_seconds % 3600) / 60;
    let seconds = duration_in_seconds % 60;

    println!("{}:{}:{}", hour, minutes, seconds);
}

fn main() {
    let seconds = handle_input();
    time_conversion(seconds);
}
