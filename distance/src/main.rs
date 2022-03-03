use std::io;

fn handle_input() -> u32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn time(distance: u32) -> u32 {
    distance * 2
}

fn main() {
    let distance = handle_input();
    println!("{} minutos", time(distance));
}
