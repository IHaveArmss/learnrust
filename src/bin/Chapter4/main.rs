// ==============================================================================
// CHAPTER 4: UNDERSTANDING OWNERSHIP - PRACTICE CHALLENGES
// ==============================================================================
//
// 4.1: WHAT IS OWNERSHIP? (MOVE SEMANTICS, CLONE, & COPY)
// ------------------------------------------------------------------------------
// Exercise 1: Move vs Copy
//   Part A (Copy Types):
//     - Create an integer `a = 10`.
//     - Assign `let b = a;`.
//     - Print both `a` and `b`. (Why does this work? Because integers implement `Copy`)
fn _ex1(){
    let a = 10;
    let b = a;
    println!("{a} {b}");
}
//   Part B (Move Types on the Heap):
//     - Create a String `s1 = String::from("Rustacean")`.
//     - Assign `let s2 = s1;`.
//     - What happens if you try to `println!("{}", s1);` now? (Try it, see the error,
//       then comment it out and explain what happened).
//     - Print `s2`.
fn _ex1_b(){
    let s1 : String = String::from("Rustacean");
    let s2 = s1;
    println!("{}",s2);
}
// Exercise 2: Deep Copy with Clone
//     - Create a String `original = String::from("Antigravity")`.
//     - Create `duplicate` by cloning `original` explicitly with `.clone()`.
//     - Modify or print both to show that both are still valid and own separate heap data.
fn _ex2(){
    let anti = String::from("Antigravity");
    let mut multi = anti.clone();    
    multi.remove(3);
    println!("{} {}",multi,anti);
}
// Exercise 3: Ownership and Functions
//     - Write a function `take_ownership(s: String)` that prints `s`.
//     - Write a function `give_ownership() -> String` that creates and returns a String.
//     - Write a function `take_and_give_back(s: String) -> String` that accepts a String
//       and returns it back.
//     - In `main`, call all three and track where ownership moves at each step.
fn _take_ownership(s : String){
    println!("{s}");
}
fn _give_ownership() -> String{
    String::from("Skibidi")
}
fn _take_and_give_back(s: String) -> String{
    s
}
fn _ex3(){
    let mut s = _give_ownership();
    s = _take_and_give_back(s);
    _take_ownership(s);
}
// ------------------------------------------------------------------------------
// 4.2: REFERENCES & BORROWING
// ------------------------------------------------------------------------------
// Exercise 4: Immutable Borrowing
//     - Write a function `calculate_length(s: &String) -> usize` that returns the length of `s`.
//     - In `main`, create a `String` and pass an immutable reference `&` to `calculate_length`.
//     - Print both the length and the original `String` to prove you can still use it.
fn _calculate_length(s:&String) -> usize{
    s.len()
}
fn _ex4(){
    let s = "sigma".to_string();
    println!("{}",_calculate_length(&s));
    println!("{}",s);
}
// Exercise 5: Mutable Borrowing
//     - Write a function `append_world(s: &mut String)` that appends `" World!"` to `s`
//       using `s.push_str(...)`.
//     - In `main`, declare a `mut greeting = String::from("Hello");`.
//     - Pass a mutable reference `&mut greeting` to `append_world`.
//     - Print `greeting`.
fn _append_world(s: &mut String){
    s.push_str(" World!");
}
fn _ex5(){
    let mut s = "Hello".to_string();
    _append_world(&mut s);
    println!("{}",s);
}

// Exercise 6: The Golden Rule of References (Aliasing XOR Mutability)
//     - Demonstrate the borrow checker rule:
//       "You can have any number of immutable references OR exactly ONE mutable reference."
//     - Create a `mut message = String::from("Hold this");`.
//     - Create two immutable references `r1 = &message;` and `r2 = &message;`.
//     - Print `r1` and `r2`.
//     - NOW create a mutable reference `r3 = &mut message;` and modify it.
//     - Notice where `r1` and `r2`'s scopes must end (Non-Lexical Lifetimes) for `r3` to be allowed!
fn _ex6(){
    let mut message = String::from("Hold this ");
    let r1 = &message;
    let r2 = &message;
    println!("{} {}",r1,r2);

    let r3 = &mut message;
    r3.push_str("string");
    println!("{}",r3);

}
// ------------------------------------------------------------------------------
// 4.3: THE SLICE TYPE
// ------------------------------------------------------------------------------
// Exercise 7: String Slices (`&str`)
//     - Create a String `phrase = String::from("Hello Ferris");`.
//     - Create a slice `hello` pointing to the first word (range `0..5`).
//     - Create a slice `ferris` pointing to the second word (range `6..12` or `6..`).
//     - Print both slices.
fn _ex7(){
    let strng = String::from("Hello Ferris");
    let slide_hello = &strng[0..5];
    let slice_ferris = &strng[6..];
    println!("{slide_hello} {slice_ferris}");

}
// Exercise 8: Writing `first_word`
//     - Write a function `first_word(s: &str) -> &str` that returns the first word of any string.
//       (Hint: Convert `s.as_bytes()`, iterate with `.iter().enumerate()`, find `b' '`,
//       and return `&s[0..i]`. If no space is found, return `&s[..]`).
//     - Test it with both a `String` (pass `&my_string[..]`) and a string literal `&"apple pie"`.
fn _first_word(s:&str) ->&str{
    let bytes = s.as_bytes();
    for (a,&b) in bytes.iter().enumerate(){
        if b.is_ascii_whitespace() {
            return &s[0..a]
        }
    }
    &s[..]
}
fn _ex8(){
    let stri = "Hello world!";
    println!("{}",_first_word(stri));
}

// Exercise 9: Array Slices
//     - Create an array `numbers = [10, 20, 30, 40, 50];`.
//     - Create a slice `middle_slice: &[i32] = &numbers[1..4];`.
//     - Print `middle_slice` using debug formatting `{:?}`.
fn _ex9(){
    let numbers = [10,20,30,40,50];
    let middle_slice :&[i32]= &numbers[0..numbers.len()/2];
    println!("{:?}",middle_slice);
}
// ==============================================================================

fn main() {

    println!("--- Chapter 4: Ownership, Borrowing & Slices ---");
    _ex9();
    // Call your exercise functions here!
}