use std::io;
use rand::{Rng, thread_rng};
use std::cmp::Ordering;

fn io_handling()->i32{
    loop{
        let mut buffer = String::new();
        io::stdin().read_line( &mut buffer).expect("error msg");
        match buffer.trim().parse::<i32>(){
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;},
        }
    }
}
fn main(){

    let number = thread_rng().gen_range(1..=100);
    loop{
        let guess = io_handling();
        match guess.cmp(&number) {
            Ordering::Equal => {println!("Numbers are equal!");
                                println!("{} : {}",guess,number); 
                                break;},
            Ordering::Less => println!("guessed number is lower than secret number"),
            Ordering::Greater => println!("Guessed number is larger than secret number"),
        }
    }
       
}

//programing a guessing game