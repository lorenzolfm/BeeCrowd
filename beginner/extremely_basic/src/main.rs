use std::io;

fn handle_input() -> i32 {
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .unwrap();

    string.trim().parse::<i32>().unwrap()
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = handle_input();
    let y = handle_input();

    println!("X = {}", sum(x, y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum(1, 2), 3);
    }
}
