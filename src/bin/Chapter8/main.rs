// ==============================================================================
// CHAPTER 8: COMMON COLLECTIONS - PRACTICE CHALLENGES
// ==============================================================================
//
// 8.1: STORING LISTS OF VALUES WITH VECTORS (`Vec<T>`)
// ------------------------------------------------------------------------------
// Exercise 1: Creating and Updating Vectors
//   - Create an empty vector `v1` of type `Vec<i32>` using `Vec::new()`.
//   - Create a vector `v2` with initial values `1, 2, 3, 4, 5` using the `vec!` macro.
//   - Create a mutable vector `v3` and use `.push()` to add numbers 10, 20, and 30.
//   - In `_ex1`, verify and print the contents of all three vectors.
fn _ex1() {
    // TODO: Create v1, v2, and v3, push elements to v3, and print them
}

// Exercise 2: Reading Elements: Indexing (`[]`) vs `.get()`
//   - Create a vector `v = vec![100, 200, 300, 400, 500]`.
//   - Read the third element using indexing syntax: `&v[2]`.
//   - Read the third element safely using `v.get(2)` and handle the `Option<&i32>`
//     with a `match` or `if let`.
//   - Try accessing index 100 with `v.get(100)` and show how it returns `None` instead
//     of panicking.
//   - (Optional: see what happens if you access `&v[100]` with direct indexing!).
fn _ex2() {
    // TODO: Compare direct indexing vs safe .get() access with bounds checking
}

// Exercise 3: Borrow Checker & Vector Mutation (The Push vs Reference Trap)
//   - Create a mutable vector `v = vec![1, 2, 3]`.
//   - Obtain an immutable reference to the first element: `let first = &v[0];`.
//   - Now try pushing a new value: `v.push(4);`.
//   - Then try using `println!("First element is: {first}");`.
//   - Observe the compiler error! Why does adding an element to the end invalidate
//     a reference to the first element? (Hint: heap reallocation & pointer invalidation).
//   - Fix the code by reordering statements or scoping so the immutable borrow ends
//     before `v.push()` is called.
fn _ex3() {
    // TODO: Demonstrate and resolve the borrow checker rule between &v[0] and v.push()
}

// Exercise 4: Iterating Over Vectors (Immutable & Mutable)
//   - Part A: Given `v = vec![10, 20, 30]`, loop over `&v` using `for i in &v` and print each item.
//   - Part B: Given a mutable vector `v_mut = vec![1, 2, 3]`, loop over mutable references
//     `for i in &mut v_mut` and add 50 to each element using the dereference operator `*`.
//   - Print `v_mut` afterwards to verify the mutation.
fn _ex4() {
    // TODO: Iterate over immutable references, then mutate elements via &mut and `*`
}

// Exercise 5: Using an Enum to Store Multiple Types in a Vector
//   - Vectors can only store values of the same type. But when you need to store
//     different types, you can define an enum whose variants hold different data types!
//   - Define an enum `SpreadsheetCell` with variants:
//       - `Int(i32)`
//       - `Float(f64)`
//       - `Text(String)`
//   - In `_ex5`, create a `row: Vec<SpreadsheetCell>` containing an Int(3), Text("blue"),
//     and Float(10.12).
//   - Iterate through `row` and print each cell using pattern matching with `match`.
fn _ex5() {
    // TODO: Define SpreadsheetCell enum and store heterogeneous data inside a Vec
}

// Exercise 6: Vector Slicing and In-Place Removal
//   - Create a vector `mut nums = vec![10, 20, 30, 40, 50]`.
//   - Use `.pop()` to remove and return the last element (`Option<T>`).
//   - Use `.remove(1)` to remove the element at index 1 (shifting remaining elements left).
//   - Take a subslice `&nums[0..2]` and print it.
//   - Print the final vector and observe its contents and length (`.len()`).
fn _ex6() {
    // TODO: Practice .pop(), .remove(), slicing, and inspecting .len()
}

// ------------------------------------------------------------------------------
// 8.2: STORING UTF-8 ENCODED TEXT WITH STRINGS (`String` & `&str`)
// ------------------------------------------------------------------------------
// Exercise 7: Creating Strings & Appending (`push_str` vs `push`)
//   - Create a String using `String::new()`, `String::from("hello")`, and `"hello".to_string()`.
//   - Create `mut s = String::from("foo")`.
//   - Use `s.push_str("bar")` to append a string slice without taking ownership.
//   - Use `s.push('!')` to append a single character `char`.
//   - In `_ex7`, print the final string and demonstrate that `push_str` didn't consume
//     the slice passed to it.
fn _ex7() {
    // TODO: Create strings and mutate them with push_str (&str) and push (char)
}

// Exercise 8: String Concatenation (`+` Operator vs `format!`)
//   - Part A: Given `let s1 = String::from("Hello, ");` and `let s2 = String::from("world!");`:
//       - Concatenate them using `let s3 = s1 + &s2;`.
//       - Notice: which string was moved, and which was borrowed? (Can `s1` still be used? What about `s2`?).
//   - Part B: For multiple strings (`tic`, `tac`, `toe`), use the `format!("{tic}-{tac}-{toe}")`
//     macro to combine them cleanly without taking ownership of any operands.
//   - In `_ex8`, print both resulting strings.
fn _ex8() {
    // TODO: Explore concatenation mechanics with + and the format! macro
}

// Exercise 9: Indexing into Strings & UTF-8 Internal Representation
//   - In Rust, `s[0]` does NOT compile! Why? (Strings are `Vec<u8>` wrappers, and UTF-8 characters
//     can be 1 to 4 bytes long; index by byte != index by character).
//   - Consider the Cyrillic string: `let hello = "Здравствуйте";`.
//   - Check `hello.len()`: notice it's 24 bytes, not 12 characters!
//   - Use string slicing `&hello[0..4]` to grab the first two 2-byte characters ("Зд").
//   - (Caution: what would happen if you sliced `&hello[0..1]`? It panics at runtime at a character boundary!).
//   - Iterate through characters using `.chars()` and print them.
//   - Iterate through raw bytes using `.bytes()` and print them.
fn _ex9() {
    // TODO: Inspect UTF-8 byte lengths, safe character slicing, .chars(), and .bytes()
}

// ------------------------------------------------------------------------------
// 8.3: STORING KEYS WITH ASSOCIATED VALUES IN HASH MAPS (`HashMap<K, V>`)
// ------------------------------------------------------------------------------
// Exercise 10: Basic HashMap Creation, Insertion, and Retrieval
//   - Import `use std::collections::HashMap;`.
//   - Create a new `HashMap`: scores of teams in a game.
//   - Insert "Blue" -> 10, "Yellow" -> 50.
//   - Retrieve the score for "Blue" using `.get(&key)` and `.copied().unwrap_or(0)`.
//   - Iterate over all key-value pairs using `for (key, value) in &scores` and print them.
fn _ex10() {
    // TODO: Create HashMap, insert key-value pairs, query values, and iterate
}

// Exercise 11: HashMap Ownership Rules
//   - For types that implement the `Copy` trait (like `i32`), values are copied into the hash map.
//   - For owned types like `String`, the keys and values are MOVED and the hash map becomes
//     the owner of those values!
//   - In `_ex11`:
//       - Create `field_name = String::from("Favorite color")` and `field_value = String::from("Blue")`.
//       - Insert them into a `HashMap<String, String>`.
//       - Try printing `field_name` after insertion (verify compiler error, then comment it out).
//       - Alternatively, show how inserting references (`&String` or `&str`) works with lifetimes.
fn _ex11() {
    // TODO: Demonstrate ownership transfer when inserting Strings into a HashMap
}

// Exercise 12: Updating a Hash Map (Overwriting, Entry API, and Mutating in Place)
//   - Part A (Overwriting):
//       - Insert a key with a value, then insert the same key with a different value.
//         Verify the old value is replaced.
//   - Part B (Adding Key/Value only if Key isn't present):
//       - Use `scores.entry(String::from("Yellow")).or_insert(50);`.
//       - Use `scores.entry(String::from("Blue")).or_insert(50);`.
//       - Observe that `or_insert` only inserts if the entry does not already exist!
//   - Part C (Updating a Value based on the Old Value):
//       - Count the frequency of each word in the string:
//         "hello world wonderful world hello hello"
//       - Use `.split_whitespace()` and `.entry(word).or_insert(0)` which returns a mutable
//         reference `&mut i32`.
//       - Dereference `*count += 1` to increment the tally.
//       - Print the word counts map.
fn _ex12() {
    // TODO: Implement overwriting, the .entry().or_insert() API, and word frequency counting
}

// ------------------------------------------------------------------------------
// 8.4: CHAPTER 8 SUMMARY CHALLENGES (THE BOOK EXERCISES)
// ------------------------------------------------------------------------------
// Challenge 1: Median and Mode of an Integer List
//   - Write a function `median_and_mode(numbers: &[i32]) -> (f64, i32)`:
//       - Median: Sort the list. If length is odd, take the middle element.
//         If even, take the average of the two middle elements.
//       - Mode: Count occurrences of each number using a `HashMap`, then find the key
//         with the maximum count.
//   - In `_challenge1`, test with a sample list e.g. `[42, 1, 36, 12, 36, 5, 24, 36, 8]`.
fn _challenge1() {
    // TODO: Calculate median and mode using Vec and HashMap
}

// Challenge 2: Pig Latin Converter
//   - Write a function `to_pig_latin(text: &str) -> String`:
//       - Rules:
//           - If a word begins with a consonant, move the first consonant to the end
//             and add "ay" (e.g. "first" -> "irst-fay", "banana" -> "anana-bay").
//           - If a word begins with a vowel (a, e, i, o, u), add "-hay" to the end
//             (e.g. "apple" -> "apple-hay", "eat" -> "eat-hay").
//       - Keep in mind UTF-8 character boundaries and handling multiple words!
//   - In `_challenge2`, convert "apple first banana eat world" and print the result.
fn _challenge2() {
    // TODO: Convert words into Pig Latin following UTF-8 safe text transformations
}

// Challenge 3: Company Employee Directory CLI / Query Engine
//   - Using a `HashMap<String, Vec<String>>`, create a system to manage employees by department.
//   - Implement operations:
//       1. Add an employee to a department: e.g. "Add Sally to Engineering", "Add Amir to Sales".
//       2. Retrieve a sorted list of all employees in a specific department.
//       3. Retrieve a list of all employees in the entire company, grouped by department
//          and sorted alphabetically (both department names and employee names).
//   - In `_challenge3`, populate several departments, query "Engineering", and print
//     the complete company directory.
fn _challenge3() {
    // TODO: Implement department-based employee directory using HashMap<String, Vec<String>>
}

// ==============================================================================

fn main() {
    println!("--- Chapter 8: Common Collections (Vectors, Strings, Hash Maps) ---");
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
    _ex12();
    _challenge1();
    _challenge2();
    _challenge3();
}
