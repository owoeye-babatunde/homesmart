# Let's Rust!

## Session 1

#### Table of contents
* [Goals](#goals)
* [Setting up the tools](#1-setting-up-the-tools)
* [Compile and rust your first program](#2-write-compile-and-run-a-hello-world-program)
* [Downloading a CSV file with `reqwest`](#how-to-download-an-external-file-with-reqwest)
* [Generic types and trait bounds](#generic-types-in-function-definitions-and-trait-bounds)
* [Challenges](#challenges)



### Goals

- [x] Set up the tools
    - [x] rustc
    - [x] cargo
    - [x] rust analyzer
    - [x] ide -> cursor/vscode

- [x] Write a basic Hello World app with Cargo
- [x] Compile and run it

- [x] Download external CSV file to disk
- [ ] Load file from disk into memory
- [ ] Prepare the data
- [ ] Train an XGBoost model with this data
- [ ] Push this model to an AWS S3 bucket (model registry)


### Setting up the tools

1. Install Rust using rustup (the official Rust installer):
   - **Windows**: Visit https://rustup.rs and download/run rustup-init.exe
   - **macOS/Linux**: Run in terminal:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

     Further information for Windows users is [here](https://rust-lang.github.io/rustup/installation/windows-msvc.html)

2. Test the installation worked
    ```bash
   rustc --version
   cargo --version
   ```

3. Install the `rust-analyzer` extension.

### Write, compile and run a Hello World program

```bash
cargo new house-price-predictor
cd house-price-predictor

# compiles and runs the program
cargo run
```

After this last command, there is a new binary file inside `target/debug/house-price-predictor`
This is a standalone binary, you can run on your computer. No need for `rustc` or `cargo` anymore.

```bash
# Run your debug binary
./target/debug/house-price-predictor
```

You generate the final optimized binary by running
```bash
cargo new --release
./target/release/house-price-predictor
```

#### Makefiles to the rescue
I recommend creating a Makefile at the root of your project to encapsulate
potentially long, repetive commands, into simple `make` instructions.

Before that, ensure you have `make` installed in your system.

- Unix/Linux/Mac -> (I think) it comes by default
- Windows. These are the instructions when using the Windows Subsysetm for Linux(WSL) 
    ```
    wsl --install
    sudo apt update
    sudo apt install make
    ```
    
### How to download an external file with `reqwest`?

- Install reqwest `cargo add reqwest` with the `blocking` feature, because by default is an `async` operation, but we don't care about async and we just want to make it work `sync`.

- reqwest::blocking::get(url) returns a Result enumeration

    ```rust
    enum Result<T, E> {
        Ok(T)
        Err(E)
    }
    ```

- To handle this Result output we use the `?` operator.

In Rust, the ? operator is a error handling operator that provides a convenient way to propagate errors. Here's what it does:
* If the operation succeeds (returns Ok(value)), it unwraps the value
* If the operation fails (returns Err(e)), it automatically returns that error from the current function

### Generic types in function definitions and **trait** bounds

#### What is the problem?
```rust
// You can add 2 integers but not two floating numbers
fn sum_two_integer_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// You can add 2 floating numbers but not 2 integers
fn sum_two_floating_numbers(a: f64, b: f64) -> f64 {
    a + b
}

let sum = sum_two_integer_numbers(1, 2);
println!("Sum of 1 and 2 is {}", sum);

let sum_2 = sum_two_floating_numbers(3.0, 4.5);
println!("Sum of 3.0 and 4.5 is {}", sum_2);

// This will not work
let sum_3 = sum_two_floating_numbers(1, 2.5);
println!("Sum of 1 and 2.5 is {}", sum_3);
```
Is there a way, to define one single function you can use to add 2 numbers, no matter
if they are integer or floating.

Yes. Generic types in your function declaration, together with trait bounds, to the rescue!


#### What is a trait in Rust?
Traits is like what other language call interfaces, or in Python, they are called abstract classes.

It declares and possibly defines a set of methods (thinks you can do) with
and type that implements that trait.

**For example:**

- i32 and f64 are different types.
- However, they also have A LOT of common functionality. For example, both can be added, substracted, multiplied, divided (not by 0 obviously).

So, to add any two numbers in rust with a custom function you can write something like this
```rust
fn sum_two_numbers<T, U, Output>(a: T, b: U) -> Output
where
    T: std::ops::Add<U, Output = Output>
{
    a + b
}
```
You read it like this:
- sum_two_numbers is a function that accepts two parameters a and b, with generic types T and U.
- These generic types T and U must be "summable". This is what we say in the where part
```T: std::ops::Add<U, Output = Output>```

You can pass any type parameters to this function that satisfy the std::Ops::Add trait.
```rust
// Call it with 2 integers and it works
let sum = sum_two_numbers(1, 2);
println!("Sum of 1 and 2 is {}", sum);

// Call it with 2 floating numbers and it works
let sum_2 = sum_two_numbers(3.0, 4.5);
println!("Sum of 3 and 4.5 is {}", sum_2);
```

### Challenges

> Update the download_csv_file function such that, instead of download one single file for the Boston Housing prices dataset, it downloads all the historical taxi rides from 2024 from the NYC taxi website?
>
> https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page
>
> Data is available month by month, so you better check how for loops work in Rust.
>
> Share your solution on Discord so we can all learn!

### Error handling in Rust

Let's see how to handle errors coming from a function called `failable_function`.

First, the fact that the function can succeed or fail is something you can quickly see
by looking at the return type of the function, which is a `Result<String, String>`

```rust
use rand::Rng;

fn failable_function() -> Result<String, String> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        Ok("Success".to_string())
    } else {
        Err("Failure".to_string())
    }
}
```

`Result` is a public enum from the Rust standard library, which looks like this:
```rust
pub enum Result<T, E> {
    /// Contains the success value
    #[lang = "Ok"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    #[lang = "Err"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
```

Let's now forget about the lines starting with #, which are called `attributes`, metadata that provide additional information to the compiler.

```rustc
pub enum Result<T, E> {
    /// Contains the success value
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
```

What this means, in plain English is that
- 