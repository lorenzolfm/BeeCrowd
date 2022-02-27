use std::io;
use std::fmt::Debug;
use std::str::FromStr;

fn handle_input<T: std::str::FromStr>() -> T 
    where <T as FromStr>::Err: Debug
{
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}


struct Employee {
    name: String,
    fixed_salary: f64,
    sales_total: f64,
}

fn get_total_salary(employee: Employee) -> f64 {
    static BONUS_FEE: f64 = 0.15;

    employee.fixed_salary + (employee.sales_total as f64 * BONUS_FEE)
}

fn main() {
    let employee = Employee {
        name: handle_input(),
        fixed_salary: handle_input(),
        sales_total: handle_input(), 
    };

    println!("TOTAL = R$ {:.2}", get_total_salary(employee));
}
