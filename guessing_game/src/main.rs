use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("We're going to play a guessing game!");

    println!("Please set the maximum value (not higher than 255):");
    let mut max = String::new();
    io::stdin()
        .read_line(&mut max)
        .expect("Failed to read user input!");
    let max: u8 = max.trim().parse().expect("Type a number smaller than or equal to 255!");

    let secret_number = rand::thread_rng().gen_range(1..=max);

    println!("Now guess the right number between 1 and {max}!");

    loop {
        println!("Type your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input!");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("You guessed too low!"),
            Ordering::Greater   => println!("You guessed too high!"),
            Ordering::Equal     => {
                println!("You guessed correctly! GG :)");
                break;
            }
        }
    }
}
