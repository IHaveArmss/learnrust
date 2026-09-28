use std::process::exit;

use crate::{Message::Quit, UsState::Alabama};


// ==============================================================================
// CHAPTER 6: ENUMS AND PATTERN MATCHING - PRACTICE CHALLENGES
// ==============================================================================
//
// 6.1: DEFINING AN ENUM
// ------------------------------------------------------------------------------
// Exercise 1: Basic Enums & Variants
//   - Define an enum `IpAddrKind` with variants:
//       - `V4`
//       - `V6`
//   - Write a function `route(ip_kind: IpAddrKind)` that prints a message indicating
//     which kind of IP address was passed.
//   - In `_ex1`, instantiate both variants and pass them to `route`.
enum IpAddrKind {
    V4,
    V6,
    Err
}
impl IpAddrKind{
    fn new(number :u8)->Self{
        match number{
            4 => IpAddrKind::V4,
            6 => IpAddrKind::V6,
            _ => IpAddrKind::Err
        }
    }
}
fn route(ip_kind: IpAddrKind){
    match ip_kind{
        IpAddrKind::V4 => println!("IPV4"),
        IpAddrKind::V6 => println!("IPV6"),
        IpAddrKind::Err => println!("ERRORRRR"),
    }
}
fn _ex1() {
    // TODO: Define IpAddrKind enum, route function, and call it with both variants
    let ip_kind = IpAddrKind::new(4);
    route(ip_kind);

}

// Exercise 2: Enums with Data Payloads (Heterogeneous Variants)
//   - Unlike structs where every instance has the same fields, enum variants
//     can each store different types and numbers of fields!
//   - Define an enum `IpAddr` with variants:
//       - `V4(u8, u8, u8, u8)`
//       - `V6(String)`
//   - In `_ex2`, create:
//       - `home`: IPv4 address 127.0.0.1
//       - `loopback`: IPv6 address "::1"
#[derive(Debug)]
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String)
}
fn _ex2() {
    // TODO: Define IpAddr enum and instantiate V4 and V6 variants
    let ipv4 = IpAddr::V4(127,0,0,1);
    let ipv6 = IpAddr::V6("::1".to_string());
    println!("{:?} {:?}",ipv4,ipv6);
    match ipv4 {                                                                                                                                               
        IpAddr::V4(a, b, c, d) => println!("{a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("{addr}"),
    }
}

// Exercise 3: Rich State Enum & Associated Methods (`impl`)
//   - Define a `Message` enum with variants representing different events:
//       - `Quit` (has no data)
//       - `Move { x: i32, y: i32 }` (has named fields, like an anonymous struct)
//       - `Write(String)` (contains a single String)
//       - `ChangeColor(i32, i32, i32)` (contains three integers)
//   - Add `#[derive(Debug)]` to `Message`.
//   - In an `impl Message` block, define a method:
//       - `fn call(&self)` that prints the message using `{:?}`.
//   - In `_ex3`, instantiate a `Message::Write` and call its `.call()` method.
#[derive(Debug)]
enum Message{
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
    Quit,
}
impl Message{
    fn call(&self){
        match &self{
            Message::Move{ x,y} => println!("Moved to {x} {y}"),
            Message::ChangeColor(R,G ,B )=> println!("Changed color to {R},{G},{B}"),
            Message::Write(message_string) => println!("{}",message_string),
            Message::Quit =>exit(100) ,
        }
    }
}
fn _ex3() {
    // TODO: Define Message enum, implement call(&self), and invoke it
    let msg = Message::Write("skibiid".to_string());
    msg.call();
}

// Exercise 4: The `Option<T>` Enum & The Absence of Null
//   - Rust does not have `null`. Instead, the standard library defines:
//       enum Option<T> {
//           Some(T),
//           None,
//       }
//     (Included automatically in the standard prelude - no import required!)
//   - In `_ex4`:
//       - Create `some_number` containing `Some(5)`.
//       - Create `absent_number` with explicit type `Option<i32>` containing `None`.
//       - Try writing `let sum = 5 + some_number;` to see why the compiler refuses it
//         (they are different types; you can't use an Option without handling the None case).
//       - Print both variables using `{:?}`.

fn _ex4() {
    // TODO: Experiment with Option<T> and notice compiler errors with +
    let some_number = Option::Some(5);
    let absent_number :Option<i32> = Option::None;
    match some_number{
        Option::Some(x) => println!("{}",x),
        Option::None => println!("None"),
    }
    println!("{:?}{:?}",some_number,absent_number);
}

// ------------------------------------------------------------------------------
// 6.2: THE `match` CONTROL FLOW CONSTRUCT
// ------------------------------------------------------------------------------
// Exercise 5: Pattern Matching with Enum Variants & Data Binding
//   - Define an enum `UsState` with a few states (e.g. `Alabama`, `Alaska`, `California`).
//     Derive `Debug` for `UsState`.
//   - Define an enum `Coin`:
//       - `Penny`
//       - `Nickel`
//       - `Dime`
//       - `Quarter(UsState)`
//   - Write a function `value_in_cents(coin: &Coin) -> u32` that:
//       - Returns 1 for Penny, 5 for Nickel, 10 for Dime.
//       - For `Quarter(state)`, prints `"State quarter from {:?}!"` and returns 25.
//   - In `_ex5`, test your function with a Penny and an Alaska Quarter.
#[derive(Debug)]
enum UsState{
    Alabama,
    California,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl Coin{
    fn value_in_cents(&self)->u32{
        match &self{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state)=>{
                println!("Coin from state {:?}",state);
                25
            }
        }
    }
}
fn _ex5() {
    // TODO: Define UsState, Coin, implement value_in_cents(), and test it
    let banut = Coin::Quarter(Alabama);
    println!("{}",banut.value_in_cents());
}

// Exercise 6: Matching with `Option<T>`
//   - Write a function `plus_one(x: Option<i32>) -> Option<i32>` that:
//       - Matches on `x`.
//       - If `None`, returns `None`.
//       - If `Some(i)`, returns `Some(i + 1)`.
//   - Remember: match arms must be exhaustive! What happens if you forget `None`?
//   - In `_ex6`, pass `Some(5)` and `None` to `plus_one` and print the returned options.
fn plus_one(x : Option<i32>)-> Option<i32>{
    match x{
        Option::Some(number) => Option::<i32>::Some(number+1),
        Option::None => Option::<i32>::None,
    }
}
fn _ex6() {
    // TODO: Implement plus_one and test with Some(5) and None
    println!("{:?}",plus_one(Option::<i32>::Some(5)));
    println!("{:?}",plus_one(Option::<i32>::None));
}

// Exercise 7: Exhaustiveness & Catch-All Patterns (`_` and variables)
//   - Rust match expressions must cover every possible outcome.
//   - Write a function `evaluate_roll(dice_roll: u8)`:
//       - If 3: print "You win a fancy hat!"
//       - If 7: print "You lose your fancy hat!"
//       - Any other number: print "Move {other} spaces!", binding the value to variable `other`.
//   - Write a second version `evaluate_roll_ignore(dice_roll: u8)` where you use the `_` placeholder
//     and the unit value `()` to do nothing for all other numbers.
//   - In `_ex7`, test both functions with 3, 7, and 9.
fn evaluate_roll(dice_roll: u8){
    match dice_roll{
        3 =>println!("You win a fancy hat"),
        7 =>println!("You lose your fancy hat"),
        other => println!("Move {other} spaces."),
    };
}
fn evaluate_roll_ignore(dice_roll: u8){
    match dice_roll{
        3 =>println!("You win a fancy hat"),
        7 =>println!("You lose your fancy hat"),
        _ => (),
    };
}
fn _ex7() {
    // TODO: Implement evaluate_roll with a named catch-all and with `_ => ()`

    evaluate_roll(3);
    evaluate_roll(20);
    evaluate_roll_ignore(20);
}

// Exercise 8: Ownership & Borrowing in Pattern Matching
//   - Memory & Ownership Teacher's Insight:
//     When matching an enum holding heap data (like `Message::Write(String)`):
//     - Matching by value (`match msg`) MOVES the inner String out, invalidating `msg`.
//     - Matching by reference (`match &msg`) BORROWS the inner data as `&String`, leaving `msg` intact!
//   - In `_ex8`:
//       - Instantiate `let msg = Message::Write(String::from("Important memo"));`
//       - Match on `&msg` (by reference).
//       - In the `Message::Write(text)` arm, print `text`.
//       - After the match, print `msg` again with `println!("{:?}", msg);` to prove
//         that ownership was NOT moved!

fn _ex8() {
    // TODO: Demonstrate matching by reference &msg to preserve ownership
    let msg = Message::Write(String::from("Important memo"));
    match &msg{
        Message::Write(text)=>println!("{}",text),
        Message::ChangeColor(R,G,B) =>println!("{R}{G}{B}"),
        Message::Move { x, y }=> println!(" movd to {}:{}",x,y),
        Message::Quit => (),
    }
    println!("{:?}",msg);

}

// ------------------------------------------------------------------------------
// 6.3: CONCISE CONTROL FLOW WITH `if let` & `let else`
// ------------------------------------------------------------------------------
// Exercise 9: Concise Matching with `if let` and `else`
//   - When you only care about ONE specific variant and want to ignore the rest,
//     a full `match` expression with `_ => ()` is overly verbose.
//   - In `_ex9`:
//       - Declare `let config_max: Option<u8> = Some(3u8);`.
//       - Use `if let Some(max) = config_max { ... } else { ... }` to print the maximum
//         or print that no max was set.
fn _ex9() {
    // TODO: Write an `if let` expression with an `else` fallback
    let config_max:Option<u8> = Some(3u8);
    if let Some(max) = config_max{
        println!("Max is :{}",max);
    }
    else{
        println!("No max was set");
    }
}

// Exercise 10: Early Returns with `let else` (Modern Idiomatic Rust)
//   - In modern Rust (Edition 2021+), `let else` lets you extract data or return early,
//     preventing deep nesting and "rightward drift".
//   - Write a function `extract_login_token(header: Option<&str>) -> &str`:
//       - Use `let Some(token) = header else { return "GUEST_TOKEN"; };`
//       - Return `token`.
//   - In `_ex10`, test `extract_login_token` with `Some("bearer_jwt_xyz")` and `None`.
fn extract_login_token(header :Option<&str>)->&str{
    let Some(token) = header else{
        return "GUEST_TOKEN";
    };
    token
}
fn _ex10() {
    // TODO: Implement extract_login_token using `let else` and test it
    println!("{}",extract_login_token(Option::<&str>::Some("skibidi-toilet")));

}

// ==============================================================================

fn main() {
    println!("--- Chapter 6: Enums and Pattern Matching ---");
    // Call your exercise functions here as you complete them:
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
}
