use colored::*;
use std::io;
use std::cmp::Ordering;

pub fn run(secret_number: u32){
   loop {
       
        println!("Guess the number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,

        };

        println!("You have guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Almost there".yellow()),
            Ordering::Greater => println!("{}", "Umeenda sana jameniii!!!! ".blue()),
            Ordering::Equal => {
                println!("{}", "You win".red().bold());
                break;
            }
        }
   } 

}
