use rand::Rng;
use std::io;

fn main() {
    let x:u8 = rand::thread_rng().gen_range(1..101);
    println!("Take a guess(1-100):");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Please input a number!");
    
    println!("Your guess was {}, which is {} than the number",)
    
}
