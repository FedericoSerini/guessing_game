use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101); // inclusivo lower bound, esclusivo upper bound (1-100)

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // le variabili sono immutabili, la keyword mut le rende mutabili. String::new UTF-8 default

        io::stdin()
            .read_line(&mut guess) // read_line ritorna io.Result che è semplicemente un enum con OK e Err
            .expect("Failed to read line"); // Se io.Result equivale Err, verrà presentato questo messaggio. 

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { // _ equivale a Exception (Java) 
                println!("Inserted input is not a parseable number");
                continue;
            }
        }; 
    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
