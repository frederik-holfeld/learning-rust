use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering::{Less, Greater, Equal};

fn main() {
    println!("We're going to play a guessing game!");
    println!("Please set the maximum value (not higher than 255):");
    
    let max: u8 = loop {
        let mut max = String::new();
        stdin()
            .read_line(&mut max)
            .expect("Failed to read user input!");
        match max.trim().parse() {
            Ok(num) => break num,
            Err(_)  => println!("You need to enter a number smaller than 255!")
        }
    };

    let secret_number = rand::thread_rng().gen_range(0..=max);
    println!("Now guess the right number between 0 and {max}!");

    loop {
        println!("Type your guess:");
        
        let guess: u8 = loop {
            let mut guess = String::new();
            stdin()
                .read_line(&mut guess)
                .expect("Failed to read user input!");
            match guess.trim().parse() {
                Ok(num) => break num,
                Err(_)  => println!("You need to enter a number between 0 and {max}!")
            }
        };
        
        match guess.cmp(&secret_number) {
            Less    => println!("You guessed too low!"),
            Greater => println!("You guessed too high!"),
            Equal   => {
                println!("You guessed correctly! GG :)");
                break;
            }
        }
    }
}
