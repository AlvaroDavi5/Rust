mod modules;
use modules::arithmetics;

fn main() {
    calculate();
}

fn calculate() {
    let a: f64 = 10.0;
    let b: f64 = 5.0;

    let sum_result: f64 = arithmetics::sum(a, b);
    println!("The sum of {} and {} is {}", a, b, sum_result);

    let subtract_result: f64 = arithmetics::subtract(a, b);
    println!("The subtraction of {} and {} is {}", a, b, subtract_result);

    let multiply_result: f64 = arithmetics::multiply(a, b);
    println!(
        "The multiplication of {} and {} is {}",
        a, b, multiply_result
    );

    let divide_result: f64 = arithmetics::divide(a, b);
    println!("The division of {} and {} is {}", a, b, divide_result);
}
