// Read a code of a product 1, 
// the number of units of product 1, 
// the price for one unit of product 1.
//
// Same for product two
// Compute total

use std::io;

fn handle_input() -> (u32, u32, f64) {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let str_vec: Vec<&str> = string
        .trim()
        .split(" ")
        .collect();

    let num_vec: Vec<f64> = str_vec.into_iter().map(|str_num| str_num.parse().unwrap()).collect();
    let input_values: (u32, u32, f64) = (num_vec[0] as u32, num_vec[1] as u32, num_vec[2]);

    input_values
}

struct Sale {
    code: u32,
    units: u32,
    price: f64,
}

fn get_total(first_sale: Sale, second_sale: Sale) -> f64 {
    (first_sale.units as f64 * first_sale.price) +
        (second_sale.units as f64 * second_sale.price)
}

fn main() {
    let first_user_input = handle_input();
    let first_sale = Sale {
        code: first_user_input.0,
        units: first_user_input.1,
        price: first_user_input.2,
    };

    let second_user_input = handle_input();
    let second_sale = Sale {
        code: second_user_input.0,
        units: second_user_input.1,
        price: second_user_input.2,
    };

    println!("VALOR A PAGAR: R$ {:.2}", get_total(first_sale, second_sale));
}
