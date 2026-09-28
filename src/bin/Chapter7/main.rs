
// ==============================================================================
// CHAPTER 7: MANAGING GROWING PROJECTS WITH PACKAGES, CRATES, AND MODULES
// ==============================================================================
//
// 7.1 & 7.2: PACKAGES, CRATES, AND MODULE DEFINITIONS
// ------------------------------------------------------------------------------
// Exercise 1: Inline Module Tree and Hierarchy
//   - In Rust, modules let you organize code into groups and control privacy.
//   - Define an inline module named `sound`:
//       - Inside `sound`, define a submodule named `instrument`.
//       - Inside `instrument`, write a public function `clarinet()`.
//         It should print "Clarinet is playing!".
//   - In `_ex1`, call `clarinet()` from outside the `sound` module using its full path.
mod sound{
    pub mod clarinet{
        pub fn clarinet(){
            println!("Clarinet is playing");
        }
    }
    pub mod guitar{
        pub fn guitar(){
            println!("guitar is playing");
        }
    }
}

fn _ex1() {
    // TODO: Define sound::instrument::clarinet() and invoke it here.
    sound::guitar::guitar();
    sound::clarinet::clarinet();

}

// Exercise 2: Privacy Boundaries (`pub` on modules and items)
//   - In Rust, all items (functions, methods, structs, enums, modules) are PRIVATE
//     to parent modules by default!
//   - An item is only accessible to an outside module if:
//       1. Every ancestor module along the path is marked `pub`.
//       2. The item itself is marked `pub`.
//     (Items can always see items in their ancestor / parent modules).
//   - Define a module `plant_nursery`:
//       - A private function `soil_ph()` that prints "pH: 6.5".
//       - A public module `greenhouse`:
//           - A public function `water_plants()` that prints "Watering plants...".
//           - Inside `water_plants`, call the parent module's private `soil_ph()`
//             (Notice that child modules can access private items in ancestor modules!).
//   - In `_ex2`, call `plant_nursery::greenhouse::water_plants()`.
//     (Try calling `soil_ph()` directly in `_ex2` to verify the compiler stops you!).
mod plant_nursery{
    fn soil_ph(){
        println!("PH:6.5");
    }
    pub mod greenhouse{
        use crate::plant_nursery::soil_ph;

        pub fn water_plants(){
            println!("Watering plants...");
            soil_ph();
        }

    }
}
fn _ex2() {
    // TODO: Define plant_nursery, test child-to-parent privacy, and call water_plants()
    plant_nursery::greenhouse::water_plants();
}

// ------------------------------------------------------------------------------
// 7.3: PATHS FOR REFERRING TO AN ITEM IN THE MODULE TREE (`super` & `crate`)
// ------------------------------------------------------------------------------
// Exercise 3: Relative Paths with `super`
//   - `super` allows constructing relative paths starting from the parent module,
//     similar to `..` in filesystem paths.
//   - Define a top-level helper function `deliver_order()` that prints "Order delivered!".
//   - Define a module `back_of_house`:
//       - Inside `back_of_house`, define a function `cook_order()` that prints "Cooking order...".
//       - Inside `cook_order()`, call `deliver_order()` using `super::deliver_order()`.
//   - In `_ex3`, call `back_of_house::cook_order()`.
fn deliver_order(){
    println!("order delivered!");
}
mod back_of_house{
    pub fn cook_order(){
        println!("Cooking order");
        super::deliver_order();
    }
}
fn _ex3() {
    // TODO: Implement super:: relative call from back_of_house to crate root deliver_order()
    back_of_house::cook_order();
}

// Exercise 4: Struct Privacy (Public vs Private Fields)
//   - If you make a struct `pub`, its fields remain PRIVATE by default!
//   - You must mark each field you want public with `pub` explicitly.
//   - If a struct has ANY private field, it CANNOT be constructed directly with struct
//     literal syntax from outside its module; it MUST provide a public constructor (like `new`)!
//   - Define a module `restaurant`:
//       - Define `pub struct Breakfast`:
//           - `pub toast: String` (customer can choose toast type)
//           - `seasonal_fruit: String` (private field! Chef chooses season fruit)
//       - In `impl Breakfast`:
//           - Implement `pub fn summer(toast: &str) -> Breakfast` that initializes
//             `seasonal_fruit` to "peaches" and returns the Breakfast.
//   - In `_ex4`:
//       - Order breakfast using `Breakfast::summer("Rye")`.
//       - Change the toast to "Wheat".
//       - (Try modifying `meal.seasonal_fruit` directly to verify it doesn't compile!).
//       - Print the toast type.
mod restaurant{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Self{
            Breakfast { toast: (toast.to_string()), 
                        seasonal_fruit: ("peaches".to_string()) }
        }
        pub fn print_summer(&self){
            println!("{} {}",self.toast,self.seasonal_fruit);
        }
    }
}
fn _ex4() {
    // TODO: Create Breakfast struct with mixed field privacy and test field access
    let mut morning_breakfast = restaurant::Breakfast::summer("Rye");
    morning_breakfast.toast  = "Skibidi".to_string();
    morning_breakfast.print_summer();
}

// Exercise 5: Enum Privacy (Public Variants by Default)
//   - Unlike structs, if you make an enum `pub`, ALL of its variants automatically
//     become public!
//   - Define a module `menu`:
//       - Define `pub enum Appetizer`:
//           - `Soup`
//           - `Salad`
//   - In `_ex5`:
//       - Instantiate an `Appetizer::Soup` and `Appetizer::Salad` directly.
//       - Print or match on them.
mod menu{
    #[derive(Debug)]
    pub enum Appetizer{
        Soup,
        Salad,
    }
}
fn _ex5() {
    // TODO: Define public enum in a module and observe that variants are automatically public
    let a : menu::Appetizer = menu::Appetizer::Salad;
    println!("{:?}",a);
}

// ------------------------------------------------------------------------------
// 7.4: BRINGING PATHS INTO SCOPE WITH `use`
// ------------------------------------------------------------------------------
// Exercise 6: Bringing Modules vs Functions into Scope (Idiomatic Paths)
//   - Idiomatic Rust convention:
//       - For FUNCTIONS: Bring the parent MODULE into scope with `use`, then call `module::func()`.
//         This makes it clear the function isn't locally defined.
//       - For STRUCTS, ENUMS, & OTHER ITEMS: Bring the ITEM ITSELF directly into scope with `use`.
//   - Define a module `traffic`:
//       - `pub mod signals { pub fn green_light() { println!("GO!"); } }`
//       - `pub struct Vehicle { pub model: String }`
//   - In `_ex6`:
//       - Bring the module `signals` into scope: `use traffic::signals;`
//       - Bring `Vehicle` directly into scope: `use traffic::Vehicle;`
//       - Call `signals::green_light()` and create a `Vehicle`.
mod traffic{
    pub mod signals{
        pub fn green_light(){
            println!("GO");
        }
    }
    pub struct Vehicle{
        pub model: String
    }
}
use traffic::Vehicle;

use traffic::signals;
fn _ex6() {
    // TODO: Practice idiomatic `use` conventions for functions vs types
    signals::green_light();
}

// Exercise 7: Resolving Name Conflicts with the `as` Keyword
//   - When bringing two different items with the same name into the same scope,
//     use `as` to provide a local alias!
//   - Bring `Result` from `std::fmt::Result` into scope aliased as `FmtResult`.
//   - Bring `Result` from `std::io::Result` into scope aliased as `IoResult`.
//   - In `_ex7`, write dummy functions or instantiate types using `FmtResult` and `IoResult`
//     without compiler collision.
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
fn _ex7() {
    // TODO: Use `as` to disambiguate two imported types with the same name
    let a: FmtResult = FmtResult::Ok(());
    let b: IoResult<i32> = IoResult::Ok(5);

}

// Exercise 8: Re-exporting Names with `pub use`
//   - When you bring a name into scope with `use`, the new name is private to that scope.
//   - By combining `pub use`, you bring the item into scope AND make it available for
//     external callers to bring into their scope. This is called "re-exporting".
//   - Define a module `audio`:
//       - Define a submodule `effects` with `pub fn reverb() { println!("Reverberating!"); }`.
//       - In `audio`, re-export `reverb`: `pub use self::effects::reverb;`.
//   - In `_ex8`:
//       - Call `audio::reverb()` directly without needing to mention the inner `effects` module!
mod audio{
    pub mod effects{
        pub fn reverb(){
            println!("Reverbarating");
        }
    }
    pub use self::effects::reverb;
}
fn _ex8() {
    // TODO: Demonstrate `pub use` re-exporting to present an ergonomic public API
    audio::reverb();
}

// Exercise 9: Nested Paths and the Glob (`*`) Operator
//   - Part A (Nested Paths):
//       - Instead of:
//           use std::cmp::Ordering;
//           use std::cmp::min;
//         Combine them using nested path syntax: `use std::cmp::{min, Ordering};`.
//       - Combine `std::io` and `std::io::Write` using `self`: `use std::io::{self, Write};`.
//   - Part B (Glob Operator):
//       - Bring all items from standard collection module or a custom math module into scope
//         using the `*` operator (e.g. `use math::*`).
//   - In `_ex9`, test both nested imports and glob usage.
use std::cmp::{min,Ordering};
use std::io::{self,Write};
use rand::*;
fn _ex9() {
    // TODO: Clean up repetitive import statements using nested paths `{}` and glob `*`
}

// ------------------------------------------------------------------------------
// 7.5: MULTI-FILE MODULE STRUCTURE
// ------------------------------------------------------------------------------
// Exercise 10: Multi-File Module Architecture
//   - Rust allows splitting modules across files using `mod module_name;`.
//   - In modern Rust (2018+ edition):
//       - In crate root (`main.rs`): `mod storage;` looks for `storage.rs` or `storage/mod.rs`.
//       - Submodules inside `storage`: `mod disk;` looks for `storage/disk.rs`.
//   - Define an inline or separated module structure simulating a cache service:
//       - Define `mod cache`:
//           - Inside it, define a submodule `memory`:
//               - Function `pub fn get(key: &str) -> Option<String>`
//               - Function `pub fn set(key: &str, val: &str)`
//   - In `_ex10`, test retrieving and setting data via your module structure.
mod cache;
fn _ex10() {
    // TODO: Build and interact with a multi-level module structure
    cache::memory::get("Skibiid");
    cache::memory::set("Toilet","Skibidi");
}

// ==============================================================================

fn main() {
    println!("--- Chapter 7: Managing Growing Projects with Packages, Crates, and Modules ---");
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
}
