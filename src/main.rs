use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut chances: u32 = rand::thread_rng().gen_range(1..10);

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if chances == 0 {
                    println!("{}", "You lose!".red());
                    break;
                } else {
                    println!("Too small! \nYou have: {} chances", chances)
                }
            },
            Ordering::Greater => {
                if chances == 0 {
                    println!("{}", "You lose!".red());
                    break;
                } else {
                    println!("Too big! \nYou have: {} chances", chances)
                }
            },
            Ordering::Equal => {
                println!("{}", "You win!".cyan());
                break;
            }
        }
        chances = chances - 1;
    }
}