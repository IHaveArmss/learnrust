// ==============================================================================
// CHAPTER 3: COMMON PROGRAMMING CONCEPTS - PRACTICE CHALLENGES
// ==============================================================================
//
// 3.1: VARIABLES, MUTABILITY & SHADOWING
// ------------------------------------------------------------------------------
// Exercise 1: Fuel Tank (Mutability)
//   Declare a variable `fuel_level` starting at 100.
//   Simulate burning 25 units of fuel by updating it directly (using `mut`).
//   Print the updated fuel level.
fn _ex1(){
    let mut fuel_level = 100;
    fuel_level -=25;
    println!("{}",fuel_level);
}
// Exercise 2: Server Settings (Constants)
//   Declare a constant `MAX_LOGIN_ATTEMPTS` with value 5 and explicit type `u8`.
//   Declare another constant `LOCKOUT_TIME_SECONDS` calculated as `5 * 60` (`u32`).
//   Print both values in `main`.
fn _ex2(){
    const MAX_LOGIN_ATTEMPTS :u8 = 5;
    const LOCKOUT_TIME_SECONDS: u32 = 5*60;
    println!("{} {}",MAX_LOGIN_ATTEMPTS,LOCKOUT_TIME_SECONDS);
}
// Exercise 3: User Input Sanitization (Shadowing & Type Conversion)
//   1. Declare `raw_input` as a string: `"   42  "`.
//   2. Shadow `raw_input` with trimmed text using `.trim()`.
//   3. Shadow `raw_input` again by parsing it into an integer `u32` using:
//      `.parse::<u32>().expect("Not a number!")`
//   4. Shadow it one last time by adding 10 to it.
//   5. Print the final calculated number.
fn _ex3(){
    let raw_input :String = "   42 ".to_string();
    let raw_input = raw_input.trim().to_string();
    let raw_input  = raw_input.parse::<u32>().expect("Not a number");
    let raw_input = raw_input+10;
    println!("{}",raw_input);
}
// ------------------------------------------------------------------------------
// 3.2: DATA TYPES (SCALAR & COMPOUND)
// ------------------------------------------------------------------------------
// Exercise 4: Explicit Types & Operations
//   1. Perform integer division between `7` and `2` as integers (`i32`).
//   2. Perform floating point division between `7.0` and `2.0` (`f64`).
//   3. Print both results to see the difference between truncated and decimal division.
fn _ex4(){
    let a: i32 = 7/2;
    println!("{a}");
    let b:f64 = 7.0/2.0;
    println!(" {b}");
}
// Exercise 5: Tuple Destructuring & Indexing
//   Create a tuple `rgb_color` storing `(255, 128, 0, "Orange")`.
//   1. Extract the red channel (`255`) and the label (`"Orange"`) using pattern matching / destructuring (`let (...) = ...;`).
//   2. Print the green channel (`128`) using direct tuple dot notation (e.g. `.1`).
fn _ex5(){
    let tupple: (i16,i16,i16,&str) = (255,128,0,"Orange");
    let (r,g,b,name) = tupple;
    println!("{r},{g},{b},{name}");
    println!("{},{},{},{}",tupple.0,tupple.1,tupple.2,tupple.3);

}
// Exercise 6: Array Slice & Initialization
//   1. Create an array of 6 elements, all initialized to `0` using the shorthand `[0; 6]`.
//   2. Update the 3rd element (index 2) to `42`.
//   3. Print the length of the array and the updated element.
fn _ex6(){
    let mut a = [0;6];
    a[2] = "42".to_string().parse::<i32>().unwrap();
    println!("{}",a.len());
    for i in 0..a.len(){
        println!("{}",a[i]);
    }

}
// ------------------------------------------------------------------------------
// 3.3: FUNCTIONS & EXPRESSIONS VS STATEMENTS
// ------------------------------------------------------------------------------
// Exercise 7: Convert Celsius to Fahrenheit
//   Write a function `celsius_to_fahrenheit(c: f64) -> f64` outside `main`.
//   Formula: `(c * 9.0 / 5.0) + 32.0`
//   Remember: Return the value as an expression (WITHOUT a semicolon).
//   Call it in `main` with `25.0` °C and print the Fahrenheit result.
fn celsius_to_fahrenheit(c:f64)->f64{
    c * 9.0 / 5.0 +32.0
}
fn _ex7(){
    println!("{}",celsius_to_fahrenheit(25.0));
}
// Exercise 8: Block Expressions
//   In `main`, compute `total_score` using a block expression `{ ... }` that:
//   - Declares a `base = 50`
//   - Declares a `bonus = 15`
//   - Returns `base + bonus` without a semicolon.
//   Print `total_score`.
fn _ex8(){
    let total_score = {
        let base = 50;
        let bonus = 50;
        base+bonus
    };
    println!("{}",total_score);
}
// ------------------------------------------------------------------------------
// 3.4: COMMENTS
// ------------------------------------------------------------------------------
// Exercise 9: Self-documenting Code
//   Add meaningful line comments (`//`) explaining any tricky parts of your logic above.
//
// ------------------------------------------------------------------------------
// 3.5: CONTROL FLOW (IF / ELSE & LOOPS)
// ------------------------------------------------------------------------------
// Exercise 10: `if` in a `let` Statement
//   Declare a boolean `is_vip = true`.
//   Use an `if` expression to assign `discount` (type `f64`):
//   If `is_vip` is true, discount is `0.20`, otherwise `0.05`.
//   Print the discount percentage.
fn _ex10(){
    let is_vip : bool = true;
    let discount:f64;
    if is_vip==true{
        discount = 0.20;
    }
    else{
        discount = 0.05;
    }
    println!("{}",discount);
}
// Exercise 11: Retry Loop with Return Value (`loop`)
//   Simulate polling a sensor with `loop`:
//   - Declare `mut attempts = 0;`
//   - In each iteration, increment `attempts` by 1.
//   - When `attempts == 4`, `break` and return the string `"Sensor ready!"` from the loop.
//   - Assign the result of the loop to a variable `status` and print it.
fn _ex11(){
    let mut atempts = 0;

    let status: &str = loop{
        atempts += 1;
        if atempts == 4{
            
            break "Sensor Ready!";
        }
    };
    println!("{}",status);
}
// Exercise 12: Countdown (`while`)
//   Write a `while` loop that counts down from 3 to 1, printing each number,
//   and prints `"Liftoff!"` after the loop finishes.
fn _ex12(){

    let mut countdown = (1..=3).rev();
    while let Some(number) = countdown.next(){
        println!("{number}");
    }
    println!("Liftoff!");

}
// Exercise 13: Matrix / Grid Traversal (`for` loop with ranges)
//   Use nested `for` loops with ranges (e.g. `0..3`):
//   Print coordinates `(x, y)` for a 3x3 grid (from `(0,0)` to `(2,2)`).
//
// ==============================================================================
fn _ex13(){

    for i in 0..3{
        for j in 0..3{
            println!("({i},{j})");
        }
    }
}
fn main() {
    println!("--- Chapter 3 Practice Challenges ---");

    _ex7();    
}

// Define any helper functions for Exercise 7 here: