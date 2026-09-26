// ==============================================================================
// CHAPTER 5: USING STRUCTS TO STRUCTURE RELATED DATA - PRACTICE CHALLENGES
// ==============================================================================
//
// 5.1: DEFINING AND INSTANTIATING STRUCTS
// ------------------------------------------------------------------------------
// Exercise 1: Classic Structs & Field Init Shorthand
//   - Define a `User` struct with fields:
//       - `active: bool`
//       - `username: String`
//       - `email: String`
//       - `sign_in_count: u64`
//   - Write a function `build_user(email: String, username: String) -> User` that
//     uses field init shorthand to construct and return a `User` with `active = true`
//     and `sign_in_count = 1`.
//   - In `_ex1`, instantiate a mutable user, modify its email, and print fields.
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
fn build_user(email : String,username: String) -> User{
    User { active: true
        , username
        , email, 
        sign_in_count: 1 }
    
}
fn _ex1() {
    // TODO: Implement User struct and build_user helper function
    let user1 = build_user("vlad@cucea".to_string(), "skibiid".to_string());
}

// Exercise 2: Struct Update Syntax (`..`)
//   - In `_ex2`, create a `user1` using `build_user` or struct instantiation.
//   - Create a `user2` by copying `user1`'s values, but with a different email,
//     using the struct update syntax (`..user1`).
//   - Note: Notice what happens to `user1.username` vs `user1.active` (move vs copy).
fn _ex2() {
    // TODO: Demonstrate struct update syntax and observe ownership move
    let user1 = build_user("dennis@harcu".to_string(),"Skibid".to_string());
    let _user2 = User{
        email : "vlad@cucea".to_string(),
        ..user1
    };
    //ex foarte important
}

// Exercise 3: Tuple Structs
//   - Define two tuple structs:
//       - `struct Color(i32, i32, i32);`
//       - `struct Point(i32, i32, i32);`
//   - In `_ex3`, create an instance of `Color(0, 128, 255)` and `Point(10, 20, 30)`.
//   - Access their values using dot notation (`.0`, `.1`, etc.) and destructuring.
//   - Observe that `Color` and `Point` are distinct types despite having the same field types.
struct Color(i32,i32,i32);
#[derive(Clone, Copy)]
struct Point(i32,i32,i32);

fn _ex3() {
    // TODO: Create and access tuple structs
    let _new_color = Color(2,2,2);
    let _new_point = Point(20,30,10);
    let a =_new_point.clone();
    let Point(x,y,z) = _new_point;
    println!("{}{}{}",x,y,z);

}

// Exercise 4: Unit-Like Structs
//   - Define a unit-like struct: `struct AlwaysEqual;`
//   - In `_ex4`, instantiate `let subject = AlwaysEqual;`.
//   - (Unit-like structs behave like `()` and are useful when implementing traits
//     without needing to store any data).
struct AlwaysEqual;
fn _ex4() {
    // TODO: Instantiate and use a unit-like struct
    let subject = AlwaysEqual;

}

// ------------------------------------------------------------------------------
// 5.2: AN EXAMPLE PROGRAM USING STRUCTS & DEBUGGING
// ------------------------------------------------------------------------------
// Exercise 5: Area Function with Structs
//   - Define a struct `Rectangle` with `width: u32` and `height: u32`.
//   - Write a function `area(rectangle: &Rectangle) -> u32` that calculates
//     the area of the rectangle.
//   - In `_ex5`, create a `Rectangle` instance and print its area.
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
fn area(rectangle:&Rectangle) -> u32{
    rectangle.height*rectangle.width
}
fn _ex5() {
    // TODO: Define Rectangle and compute area via standalone function
    println!("{}",area(&Rectangle { width: (10), height: (20) }));
}

// Exercise 6: Debug Printing with `#[derive(Debug)]` & `dbg!` Macro
//   - Add `#[derive(Debug)]` to your `Rectangle` struct.
//   - In `_ex6`:
//       - Print the rectangle using debug format `{:?}`.
//       - Print the rectangle using pretty debug format `{:#?}`.
//       - Use the `dbg!` macro to inspect an expression (e.g. `dbg!(30 * scale)`)
//         and the rectangle itself (`dbg!(&rect)`).
fn _ex6() {
    // TODO: Demonstrate {:?}, {:#?}, and dbg!()
    let rect = Rectangle { width: (10), height: (20) };
    println!("{:?}",rect);
    println!("{:#?}",rect);
    let scale = 10;
    dbg!(30*scale);
    dbg!(&rect);
}

// ------------------------------------------------------------------------------
// 5.3: METHOD SYNTAX
// ------------------------------------------------------------------------------
// Exercise 7: Defining Methods
//   - In an `impl Rectangle` block, implement an `area(&self) -> u32` method.
//   - Write a method `has_width(&self) -> bool` that returns true if `self.width > 0`.
//   - In `_ex7`, call `rect.area()` and `rect.has_width()`.
impl Rectangle{
    fn new(width: u32,height: u32)->Self{
        Rectangle { width: (width), height: (height) }
    }
    fn square(size: u32)-> Self{
        Rectangle{width:size,height:size}
    }
    
}
impl Rectangle{
    fn can_hold(&self,other: &Rectangle)->bool{
        self.width >other.width && self.height > other.height
    }
    fn has_width(&self)->bool{
        self.width>0
    }
    fn area(&self)->u32{
        self.height*self.width
    }
}
fn _ex7() {
    // TODO: Implement and call methods on Rectangle
    let new_rect = Rectangle::new(10,20);
    println!("{} {}",new_rect.has_width(),new_rect.area());
}

// Exercise 8: Methods with Multiple Parameters
//   - In `impl Rectangle`, add a method `can_hold(&self, other: &Rectangle) -> bool`.
//     A rectangle can hold another if its width and height are both strictly greater.
//   - In `_ex8`, test `rect1.can_hold(&rect2)` and `rect1.can_hold(&rect3)`.
fn _ex8() {
    // TODO: Implement can_hold and test with multiple rectangles
    let rect1 = Rectangle::new(10,10);
    let rect2 = Rectangle::new(10, 20);
    let rect3 = Rectangle::new(5,5);
    println!("{} {}",rect1.can_hold(&rect2),rect1.can_hold(&rect3));
}

// Exercise 9: Associated Functions (Constructors)
//   - In `impl Rectangle`, add an associated function `square(size: u32) -> Self`
//     that constructs a `Rectangle` where `width` and `height` are both `size`.
//   - In `_ex9`, construct a square using `Rectangle::square(25)` and print its area.
fn _ex9() {
    let rect1 = Rectangle::square(20);
}

// Exercise 10: Multiple `impl` Blocks
//   - Rust allows defining multiple `impl` blocks for the same struct.
//   - Split your `Rectangle` methods across multiple `impl Rectangle` blocks
//     to see that this is valid syntax.
fn _ex10() {
    // TODO: Demonstrate multiple impl blocks
    let rect1 = Rectangle::new(10, 10);
    println!("{}",rect1.area());
}

// ==============================================================================

fn main() {
    println!("--- Chapter 5: Using Structs to Structure Related Data ---");
    // Call your exercise functions here!
    _ex8();
}
