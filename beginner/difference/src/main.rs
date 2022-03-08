use std::io;

fn handle_input() -> i32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn difference(values: Vec<i32>) -> i32 {
    (values[0] * values[1]) - (values[2] * values[3])
}

fn main() {
    let mut input = vec![];

    for _ in 0..4 {
        input.push(handle_input());
    }

    println!("DIFERENCA = {}", difference(input))
}
