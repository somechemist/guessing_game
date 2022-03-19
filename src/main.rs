use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pick a number between 1 and 25!");

    println!("Whoever takes the highest number of turns loses");

    let secret_number = rand::thread_rng().gen_range(1, 26); //lower bound is inclusive but the upper bound is exclusive

    let mut turn = 1;
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        //let mut turn = 0;

        let mut guess = String::new(); //declaring a mutable(changeable) variable as a string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //executes the read_line. On OK it assigns the read to guess. on Err it uses the expect

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
