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

    let zero: f64 = 0.0;
    let mut near_zero: f64 = 0.01;
    near_zero = near_zero.powf(10.0);
    println!(
        "Now let's try to divide by zero ({}) and a very small number: {}",
        zero, near_zero
    );
    let div1 = std::panic::catch_unwind(|| {
        arithmetics::divide(a, zero);
    });
    match div1 {
        Ok(_) => println!("Division by zero succeeded"),
        Err(_) => println!("Division by zero caused a panic"),
    }
    let div2 = std::panic::catch_unwind(|| {
        arithmetics::divide(a, near_zero);
    });
    match div2 {
        Ok(_) => println!("Division by a very small number succeeded"),
        Err(_) => println!("Division by a very small number caused a panic"),
    }
}
