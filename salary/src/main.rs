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

#[derive(Debug)]
struct Employee {
    id: u32,
    worked_hours_by_month: u32,
    pay_per_hour: f64,
}

fn get_month_salary(employee: Employee) -> f64 {
    employee.worked_hours_by_month as f64 * employee.pay_per_hour
}

fn main() {
    let employee = Employee {
        id: handle_input(),
        worked_hours_by_month: handle_input(),
        pay_per_hour: handle_input(),
    };

    println!("NUMBER = {}", employee.id);
    println!("SALARY = U$ {:.2}", get_month_salary(employee));
}
