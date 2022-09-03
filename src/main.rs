use rand::Rng;
use std::io;

fn main() {
    let num:u8 = rand::thread_rng().gen_range(1..101);
    println!("Take a guess(1-100):");
    let mut input = String::new();

    loop{
        io::stdin()
            .read_line(&mut input)
            .expect("Please input a number!");

        if input.parse() > num {
            println!("Your guess was {}, which is greater than the number", num);
        }else if input.parse() < num {
            println!("Your guess was {}, which is lesser than the number", num);
        }else {
            println!("Your guess is correct! The number is {}", num);
            break;
        }
    }
}
