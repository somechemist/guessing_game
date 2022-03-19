// Modified the guessing game project from "The Rust Programming Language"
// Check out the book https://doc.rust-lang.org/book/
// All feedback happily welcomed
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pick a number between 1 and 25!");

    println!("Whoever takes the highest number of turns loses");

    let secret_number = rand::thread_rng().gen_range(1, 26); //lower bound is inclusive but the upper bound is exclusive

    let mut turn = 1;

    loop {
        println!("Please input your guess");

        let mut guess = String::new(); //declaring a mutable(changeable) variable as a string

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
                println!("Too small!");
                turn += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                turn += 1;
            }
            Ordering::Equal => {
                println!("You guessed it in {} turns", turn);
                break;
            }
        }
    }
}
