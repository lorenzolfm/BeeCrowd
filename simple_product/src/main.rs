use std::io;

fn handle_input() -> i32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn main() {
    let mut input = vec![];
    for _ in 0..2 {
        input.push(handle_input())
    }
    println!("PROD = {}", input[0] * input[1]);
}
