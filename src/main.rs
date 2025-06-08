use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){

 println!("Welcome to guessy , one of the most difficult rust based games");
 println!("Where numbers keep looping to make it difficult to win");
 println!("Let's see if you can win it or not");
 println!("THe game will quit once you guess the correct number");
let mut attempt=0;


loop{ 
     let secret_number =rand::thread_rng().gen_range(1..=100);
    println!("Please enter a number:");
 let mut guess = String::new();
io::stdin()
  .read_line(&mut guess)
  .expect("failed to read line");
  
  println!("you guessed {}",guess);
  attempt+=1;


 let guess: u32 = guess.trim().parse().expect("Please type a number!");

  match guess.cmp(&secret_number){
  Ordering::Less => println!("Too small"),
  Ordering::Greater => println!(" Too big"),
  Ordering::Equal => {println!("Equal");
  println!("You gave guessed it in {attempt} attempts!");
  println!("Congratulations GOodbye");
  
  break;

  }
  }
    println!("The secret number is: {secret_number}");
}
}

