use std::io;

fn area(radius: f64) -> f64 {
    3.14159 * radius * radius
}

fn round_float_to_n_decimals(number: f64, decimals: u32) -> f64 {
    let rounding_factor: f64 = (10 as i32).pow(decimals).into();
    (number * rounding_factor).round() / rounding_factor
}

fn handle_input() -> f64 {
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .unwrap();

    string.trim().parse::<f64>().unwrap()
}

fn main() {
    let radius = handle_input();
    let area = area(radius);
    let rounded_area = round_float_to_n_decimals(area, 4);

    println!("A={:.4}", rounded_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(area(2.00), 12.56636);
        assert_eq!(area(1.00), 3.14159);
        assert_eq!(round_float_to_n_decimals(area(100.64), 4), 31819.3103);
        assert_eq!(round_float_to_n_decimals(area(150.00), 4), 70685.7750);
    }

    #[test]
    fn round() {
        assert_eq!(round_float_to_n_decimals(0.123446, 4), 0.1234);
        assert_eq!(round_float_to_n_decimals(0.123456, 4), 0.1235);
        assert_eq!(round_float_to_n_decimals(0.193456, 1), 0.2);
    }
}
