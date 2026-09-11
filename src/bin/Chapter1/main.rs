use std::{intrinsics::simd::SimdAlign::Vector, range};



fn _lover(value:i32) -> String{
    if value >10{
        return "pog".to_string();
    }
    else{
        return "skibidi".to_string();
    }
}
fn _ex1(){
    let base = 10;
    let mut counter = 0 ;
    counter += 5;
    let total = counter*base;
    println!("{total}: base:{base} counter:{counter}");
}
fn grade(score:i8)->String{
    if score>90{
        return "A".to_string();
    }
    else if score>80 {
        return "B".to_string();
    }
    else {
        return "needs improvement".to_string();
    }
        
}
fn _ex2(){
    
    
}
fn main(){
    _ex2();
}

//ex1
// Topic: Variables and Mutability
// Task:
// 1. Create an immutable variable `base` with value 10.
// 2. Create a mutable variable `counter` initialized to 0.
// 3. Add 5 to `counter`.
// 4. Create a variable `total` that stores `base * counter`.
// 5. Print all three values using println!.

//ex2
// Topic: Functions, Parameters, and Return Values
// Task:
// 1. Write a function `grade(score: i32) -> String`.
//    - If score >= 90, return "A".
//    - Else if score >= 75, return "B".
//    - Otherwise, return "Needs Improvement".
// 2. In an ex2 function, call `grade` with different scores (e.g. 95, 80, 60) and print the results.