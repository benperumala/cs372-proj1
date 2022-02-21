use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        println!("Enter your guess below");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read file");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was {}", guess);

        // if guess < secret_number {
        //     println!("Higher");
        // } else if guess > secret_number {
        //     println!("Lower");
        // } else {
        //     println!("You win!");
        //     break;
        // }

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
