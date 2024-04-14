use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numb!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        let mut guess = String::new();

        // Read the input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // Convert the input to a number and handle the error, continue the loop if the input is not a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare the input with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
