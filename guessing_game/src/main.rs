use std::{cmp::Ordering, io};

use rand::{RngExt, rng};

fn main() {
    println!("Guess the number!");

    let secret_number = rng().random_range(1..=100);
    // println!("The secret number is {secret_number}");

    println!("Please input your guess:");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("please enter a number");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too high!"),
            Ordering::Less => println!("too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed {guess}");
    }
}
