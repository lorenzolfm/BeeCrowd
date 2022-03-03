use std::io;
use std::collections::HashMap;

fn handle_input() -> u32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn banknotes(value: u32) -> HashMap<u32, u32> {
    let result = HashMap::from([
        (100, 0),
        (50, 0),
        (20, 0),
        (10, 0),
        (5, 0),
        (2, 0),
        (1, 0),
    ]);
    for i in 0..value {
        match i {
            i % 100 == 0 => {
                let amount = result.entry(100);
                *amount += 1;
            }
            i % 50 == 0 => {
                let amount = result.entry(50);
                *amount += 1;
            }
            i % 20 == 0 => {
                let amount = result.entry(20);
                *amount += 1;
            }
            i % 10 == 0 => {
                let amount = result.entry(10);
                *amount += 1;
            }
            i % 5 == 0 => {
                let amount = result.entry(5);
                *amount += 1;
            }
            i % 2 == 0 => {
                let amount = result.entry(2);
                *amount += 1;
            }
            i % 1 == 0 => {
                let amount = result.entry(1);
                *amount += 1;
            }
        }
    }

    result
}

fn main() {
    println!("Hello, world!");
}
