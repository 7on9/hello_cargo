pub mod guess_the_number;

// use rand::Rng;
// use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to the Rust programming language!");
    println!("Please select a program to run:");
    println!("1. Guess the number");
    println!("2. Exit");
    loop {
        println!("Select a program to run: ");
        let mut inp_program_id: String = String::new();

        io::stdin()
            .read_line(&mut inp_program_id)
            .expect("Failed to read line");

        println!("You typed: {inp_program_id}");

        let guess: u32 = match inp_program_id.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess {
            1 => guess_the_number::guess_the_number(),
            2 => break,
            _ => continue,
        }
    }
}
