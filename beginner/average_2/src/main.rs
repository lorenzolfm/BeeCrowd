use std::io;

fn handle_input() -> f64 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn compute_average(grades: Vec<f64>) -> f64 {
    let weights = vec![2.0, 3.0, 5.0];
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

    for _ in 0..3 {
        input.push(handle_input());
    }

    println!("MEDIA = {:.1}", compute_average(input));
}
