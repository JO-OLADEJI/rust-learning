use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to MindFlix - The guessing game.");

    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    let mut n_guess: u32 = 0;

    loop {
        let mut user_guess: String = String::new();
        println!("Enter a guess: ");

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Invalid Guess!");

        n_guess += 1;

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_guess.cmp(&secret) {
            Ordering::Less => println!("Too Low :<"),
            Ordering::Greater => println!("Too High :/"),
            Ordering::Equal => {
                println!("You Win . . . :)");
                break;
            }
        }
    }

    println!("\nYou completed the game in {} guesses", n_guess);
}
