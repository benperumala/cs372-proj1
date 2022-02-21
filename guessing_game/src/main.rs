use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game!");

    // Generate a random number
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Keep looping forever
    loop {
        let mut guess = String::new();

        println!("Enter your guess below");
        io::stdin()  // Read a line from stdin and update `guess`
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was {}", guess);

        // Comparison using if conditions
        // if guess < secret_number {
        //     println!("Higher");
        // } else if guess > secret_number {
        //     println!("Lower");
        // } else {
        //     println!("You win!");
        //     break;
        // }

        // Comparison using higher-order match functionality
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}
