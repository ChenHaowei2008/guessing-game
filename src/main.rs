use rand::Rng;
use std::io;

fn main() {

    loop{
        let mut input = String::new();
        let num:u8 = rand::thread_rng().gen_range(1..101);

        println!("Welcome to guessing game!");

        loop{
            let mut guess = String::new();

            println!("Take a guess(1-100):");
    
            io::stdin()
                .read_line(&mut guess)
                .expect("An error occurred while reading the line!");
    
            let guess: u8 = match guess.trim().parse(){
                Ok(n) => n,
                Err(_) => continue,
            };
    
            if guess > num {
                println!("Your guess was {guess}, which is greater than the number");
            }else if guess < num {
                println!("Your guess was {guess}, which is lesser than the number");
            }else {
                println!("Your guess is correct! The number is {guess}");
                break;
            }
        }
        
        println!("Do you want to play again?[Y/n]");

        io::stdin()
            .read_line(&mut input)
            .expect("An error occurred while reading the line!");
        
        if input.trim().to_ascii_lowercase() == "n" {
            println!("Thanks for playing!");
            break
        }
    }
}
