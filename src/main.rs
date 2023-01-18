use std::{io, cmp::Ordering};
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the Number!");

    let mut computer_score: i32 = 0;
    let mut player_score: i32 = 0;

    loop {
        println!("What number you choose?");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "ERROR: This needs to be a valid number!".red());
                continue
            }
        };

        let secret_number: i8 = rand::thread_rng().gen_range(0..=100);

        println!("Your guess is {guess}");
        println!("Computer guess is {secret_number}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}\n\n", "Too small".red());
                computer_score += 1;
            },
            Ordering::Equal => {
                println!("{}\n\n", "You Win".green());
                player_score += 1;
            },
            Ordering::Greater => {
                println!("{}\n\n", "Too big".red());
                computer_score += 1
            },
        };

        println!("Player {player_score} x {computer_score} Computer\n")
    }   
}
