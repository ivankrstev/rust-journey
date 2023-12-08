use std::cmp::Ordering;
use std::io;
use rand::Rng;

// Guess the number game(console) with lower/higher hints
// Using random number generator, loops and match
pub fn main() {
    println!("Welcome to the guess game!");
    let random_limit: u32;
    println!("Enter the limit for the computer guess: ");
    loop {
        let mut random_number_limit = String::new();
        io::stdin()
            .read_line(&mut random_number_limit)
            .expect("Panic!!!");
        match random_number_limit.trim().parse() {
            Ok(num) => {
                random_limit = num;
                break;
            }
            Err(_) => {
                println!("Please enter a valid limit for the computer guess:");
                continue;
            }
        };
    }
    let secret_number = rand::thread_rng().gen_range(1..=random_limit);
    let mut number_of_guesses = 0;
    loop {
        number_of_guesses = number_of_guesses + 1;
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Panic!!!");
        let parsed_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("It's a match");
                let show_num_of_guesses_string = match number_of_guesses {
                    1 => "You guessed it on your first try.".to_string(),
                    _ => format!("You guessed it after {} tries.", number_of_guesses)
                };
                println!("{show_num_of_guesses_string}");
                break;
            }
        }
    }
}