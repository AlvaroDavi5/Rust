pub fn sum(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    check_division_by_zero(b);

    a / b
}

fn check_division_by_zero(b: f64) {
    if b == 0.0 {
        panic!("Division by zero is not allowed");
    }
}
