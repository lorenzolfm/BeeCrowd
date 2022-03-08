use std::io;

fn handle_input() -> (i32, i32, i32) {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let str_vec: Vec<&str> = string
        .trim()
        .split(" ")
        .collect();

    let num_vec: Vec<i32> = str_vec.into_iter()
        .map(|str_num| str_num.parse().unwrap()).collect();

    (num_vec[0], num_vec[1], num_vec[2])
}

fn greatest(a: i32, b: i32, c: i32) -> i32 {
    let greatest_a_b = (a + b + i32::abs(a - b)) / 2;
    (greatest_a_b + c + i32::abs(greatest_a_b - c)) / 2
}

fn main() {
    let (a, b, c) = handle_input();
    println!("{} eh o maior", greatest(a, b, c));
}
