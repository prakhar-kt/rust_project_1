use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number!");

    

    let rand_num = rand::thread_rng().gen_range(1..=100);

    
    
    println!("The secret number is {rand_num}");

    loop {

        println!("Please input your number!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };



        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Correct Guess");
                break;
            }
            
        }
    }

}
