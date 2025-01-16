use rand::Rng;

// This function will panic if it fails
// Not the best way to handle errors.
// Bad Rust code will panic.
fn failable_function_with_panic() -> Result<String, String> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        Ok("Success".to_string())
    } else {
        panic!("Failure");
    }
}

// This function will return an error if it fails
// This is the best way to handle errors.
// Good Rust code should be robust to errors.
fn failable_function() -> Result<String, String> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        Ok("Success".to_string())
    } else {
        Err("Failure".to_string())
    }
}

fn main() -> Result<(), String> {

    // handle the errors with a match statement
    match failable_function() {
        Ok(value) => println!("Success: {}", value),
        Err(error) => return Err(error),
    }

    // failable_function_with_panic();

    Ok(())
}
