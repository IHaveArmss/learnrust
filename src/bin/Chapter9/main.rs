use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// ==============================================================================
// CHAPTER 9: ERROR HANDLING - PRACTICE EXERCISES & CHALLENGES
// ==============================================================================
//
// 9.1: UNRECOVERABLE ERRORS WITH `panic!`
// ------------------------------------------------------------------------------
// Exercise 1: Explicit `panic!`
//   - In Rust, unrecoverable bugs cause a program to terminate immediately via `panic!`.
//   - Write a function or branch that calls `panic!("crash and burn")`.
//   - (Keep commented out or guarded so it doesn't stop the rest of your tests!).
fn _ex1() {
    println!("\n--- Exercise 1: Explicit panic! ---");
    // TODO: Trigger a panic with a custom message (commented out by default)
    // panic!("crash and burn");
    println!("(Explicit panic is commented out so the program continues)");
}

// Exercise 2: Buffer Overrun Panic & `RUST_BACKTRACE`
//   - Access an index past the end of a vector: e.g. `let v = vec![1, 2, 3]; v[99];`.
//   - In C, this is an out-of-bounds memory read (undefined behavior / security vulnerability).
//   - In Rust, it safely halts execution by panicking.
//   - Observe how setting the environment variable `RUST_BACKTRACE=1` prints the exact
//     call stack leading to the panic.
fn _ex2() {
    println!("\n--- Exercise 2: Buffer Overrun Panic ---");
    // let v = vec![1, 2, 3];
    // let _val = v[99]; // Unrecoverable index out of bounds!
    println!("(Out-of-bounds access is commented out to avoid crashing)");
}

// ==============================================================================
// 9.2: RECOVERABLE ERRORS WITH `Result<T, E>`
// ------------------------------------------------------------------------------
// Exercise 3: Handling `Result` with `match`
//   - Attempt to open a non-existent file using `File::open("hello.txt")`.
//   - `File::open` returns a `Result<File, std::io::Error>`.
//   - Use a `match` expression to inspect the result:
//       - `Ok(file)` => print "File opened successfully!"
//       - `Err(error)` => print "Problem opening the file: {error:?}"
fn _ex3() {
    println!("\n--- Exercise 3: Handling Result with match ---");
    // TODO: Call File::open and match on Ok and Err
}

// Exercise 4: Matching on Different Error Kinds (`ErrorKind`)
//   - When `File::open("hello.txt")` fails, inspect `error.kind()`:
//       - If `error.kind() == ErrorKind::NotFound`:
//           - Attempt to create the file with `File::create("hello.txt")`.
//           - Handle the `Result` of `File::create` with another match (or panic if creating fails).
//       - For any other error kind:
//           - Panic with "Problem opening the file: {other_error:?}".
//   - (Clean up after yourself by deleting the file with `fs::remove_file("hello.txt")`).
fn _ex4() {
    println!("\n--- Exercise 4: Matching on Different Error Kinds ---");
    // TODO: Match on ErrorKind::NotFound vs other errors, creating the file if not found
}

// Exercise 5: Alternatives to `match`: `unwrap_or_else` with Closures
//   - Writing nested `match` expressions can become verbose.
//   - Re-implement Exercise 4 using functional combinators:
//       `File::open("hello.txt").unwrap_or_else(|error| { ... })`
//   - Inside the closure, check if `error.kind() == ErrorKind::NotFound` and call
//     `File::create("hello.txt").unwrap_or_else(|error| { ... })`.
fn _ex5() {
    println!("\n--- Exercise 5: unwrap_or_else with Closures ---");
    // TODO: Re-write the file creation fallback cleanly using closures and unwrap_or_else
}

// Exercise 6: Shortcuts for Panic: `unwrap` and `expect`
//   - `unwrap()`: returns the value inside `Ok`, or panics with default message if `Err`.
//   - `expect("custom message")`: returns the value inside `Ok`, or panics with your custom message!
//   - Why is `expect` generally preferred over `unwrap` in production code?
//   - Demonstrate calling `.expect()` on `File::open` (with a safe filename or inside a demonstration).
fn _ex6() {
    println!("\n--- Exercise 6: Shortcuts for Panic: unwrap and expect ---");
    // TODO: Compare unwrap() and expect()
}

// ==============================================================================
// PROPAGATING ERRORS & THE `?` OPERATOR
// ------------------------------------------------------------------------------
// Exercise 7: Propagating Errors Manually with `match`
//   - Write a function `read_username_from_file_manual() -> Result<String, io::Error>`:
//       - Open "username.txt" with `File::open`.
//       - If it fails, return `Err(e)` immediately.
//       - If it succeeds, create a `mut s = String::new();`.
//       - Call `file.read_to_string(&mut s)`.
//       - If reading fails, return `Err(e)`.
//       - If reading succeeds, return `Ok(s)`.
fn _read_username_from_file_manual() -> Result<String, io::Error> {
    // TODO: Open file, read to string, and return Result using explicit match statements
    Ok(String::new())
}

fn _ex7() {
    println!("\n--- Exercise 7: Propagating Errors Manually with match ---");
    // TODO: Call _read_username_from_file_manual() and print the result
}

// Exercise 8: Propagating Errors with the `?` Operator
//   - Rewrite the logic from Exercise 7 using `?`:
//       `let mut file = File::open("username.txt")?;`
//       `file.read_to_string(&mut s)?;`
//       `Ok(s)`
//   - Notice how `?` automatically unwraps `Ok` or returns `Err` early from the enclosing function!
fn _read_username_from_file_short() -> Result<String, io::Error> {
    // TODO: Rewrite username reading using the ? operator
    Ok(String::new())
}

fn _ex8() {
    println!("\n--- Exercise 8: Propagating Errors with the ? Operator ---");
    // TODO: Call _read_username_from_file_short() and print the result
}

// Exercise 9: Chaining Method Calls After `?`
//   - You can chain expressions with `?`:
//       `let mut s = String::new();`
//       `File::open("username.txt")?.read_to_string(&mut s)?;`
//   - (Even shorter: `fs::read_to_string("username.txt")` does the whole open + read + close!).
fn _read_username_chained() -> Result<String, io::Error> {
    // TODO: Shorten even further by chaining or using fs::read_to_string
    Ok(String::new())
}

fn _ex9() {
    println!("\n--- Exercise 9: Chaining Method Calls After ? ---");
    // TODO: Call _read_username_chained() and print the result
}

// Exercise 10: Using `?` with `Option<T>`
//   - The `?` operator can also be used on `Option<T>` inside a function that returns `Option<T>`!
//   - Write a function `last_char_of_first_line(text: &str) -> Option<char>`:
//       - Use `text.lines().next()?` to get the first line (or return `None`).
//       - Use `.chars().last()` to return the last character.
fn _last_char_of_first_line(text: &str) -> Option<char> {
    // TODO: Implement using ? on Option
    None
}

fn _ex10() {
    println!("\n--- Exercise 10: Using ? with Option<T> ---");
    // TODO: Test _last_char_of_first_line with a multi-line string and an empty string
}

// ==============================================================================
// 9.3: TO `panic!` OR NOT TO `panic!` (CUSTOM TYPES FOR VALIDATION)
// ------------------------------------------------------------------------------
// Exercise 11: Encapsulating Validation in a Custom Type
//   - When you need a number between 1 and 100, checking `if value < 1 || value > 100` everywhere
//     is tedious and error-prone.
//   - Create a struct `Guess`:
//       `pub struct Guess { value: i32 }`
//   - Implement `Guess::new(value: i32) -> Result<Guess, String>`:
//       - If value is < 1 or > 100, return `Err("Guess value must be between 1 and 100.".to_string())`.
//       - Otherwise, return `Ok(Guess { value })`.
//   - Provide a getter method `pub fn value(&self) -> i32 { self.value }`.
//   - Why does keeping the `value` field private ensure no one can construct an invalid `Guess`?
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        // TODO: Validate value is between 1 and 100
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn _ex11() {
    println!("\n--- Exercise 11: Encapsulating Validation in a Custom Type ---");
    // TODO: Test creating valid and invalid Guess instances
}

// ==============================================================================
// CHAPTER 9 CHALLENGES
// ==============================================================================
//
// Challenge 1: Safe Configuration File Parser
//   - Create a function `load_config(path: &str) -> Result<Vec<(String, String)>, String>`:
//       - Open the file at `path`. If missing, return an error message: `"File not found: {path}"`.
//       - Read each line. Lines follow the format: `KEY=VALUE`.
//       - If a line is empty or starts with `#`, skip it (comment).
//       - If a line does not contain `=`, return an error: `"Malformed line: {line}"`.
//       - Split by `=` and trim whitespace from both key and value.
//       - Return `Ok(vec_of_pairs)`.
//   - Test with both a valid config sample and an invalid config sample.
fn _challenge1() {
    println!("\n--- Challenge 1: Safe Configuration File Parser ---");
    // TODO: Implement and test load_config
}

// Challenge 2: Custom Error Enum & Conversion (`From` trait)
//   - Define an enum `AppError`:
//       - `Io(io::Error)`
//       - `Parse(std::num::ParseIntError)`
//       - `Validation(String)`
//   - Implement `From<io::Error>` and `From<std::num::ParseIntError>` for `AppError`.
//   - Write a function `sum_numbers_from_file(path: &str) -> Result<i32, AppError>`:
//       - Reads file content (can produce `io::Error`).
//       - Parses lines into `i32` (can produce `ParseIntError`).
//       - Validates that no number is negative (produces `AppError::Validation`).
//       - Uses `?` on all operations! Because of the `From` implementations, Rust
//         automatically converts both `io::Error` and `ParseIntError` into `AppError`!
fn _challenge2() {
    println!("\n--- Challenge 2: Custom Error Enum & Conversion ---");
    // TODO: Implement AppError and sum_numbers_from_file
}

// ==============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Chapter 9: Error Handling (panic!, Result, and ?) ---");
    
    // Uncomment exercises as you complete them:
    _ex1();
    _ex2();
    _ex3();
    _ex4();
    _ex5();
    _ex6();
    _ex7();
    _ex8();
    _ex9();
    _ex10();
    _ex11();
    _challenge1();
    _challenge2();

    Ok(())
}
