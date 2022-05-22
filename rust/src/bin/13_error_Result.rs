// Result -> Ok : return val, Err : panic
// enum Result<T, E> {
//     Ok(T):
//     Err(E):
// }

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(divided: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(divided / divisor)
    }
}

fn run_1() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{}", safe_division(12.0, 3.0).unwrap());
    println!("{:?}", safe_division(9.0, 0.0));
    println!("{:?}", safe_division(0.0, 5.0));
}

// ---------------------------------------
// same func_1, func_2
fn func_1() -> Result(Success, Failure) {
    match operation_that_might_fail() {
        Ok(success) => success,
        Err(failure) => return Err(failure),
    }
}

fn funct_2() -> Result(Success, Failure) {
    operation_that_might_fail()?
}


// ---------------------------------------
fn main() {
    run_1();
    println!("-----");

}
