use std::io;

fn handle_input() -> f64 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn compute_average(grades: Vec<f64>) -> f64 {
    let weights = vec![3.5, 7.5];
    let mut upper = 0.0;
    let mut sum_of_weights = 0.0;
    for i in 0..grades.len() {
        let curr_weight = weights[i];
        upper += grades[i] * curr_weight;
        sum_of_weights += curr_weight;
    }

    upper / sum_of_weights
}

fn main() {
    let mut input = vec![];

    for _ in 0..2 {
        input.push(handle_input());
    }

    println!("MEDIA = {:.5}", compute_average(input));
}
