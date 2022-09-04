use rand::Rng;
use std::io;

fn main() {
    let num:u8 = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Take a guess(1-100):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please input a number!");

        let input: u8 = match input.trim().parse(){
            Ok(n) => n,
            Err(_) => continue,
        };

        if input > num{
            println!("Your guess was {}, which is greater than the number", input);
        }else if input < num {
            println!("Your guess was {}, which is lesser than the number", input);
        }else {
            println!("Your guess is correct! The number is {}", input);
            break;
        }
    }
}
