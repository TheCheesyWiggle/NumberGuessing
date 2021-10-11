use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 and 10");

    // Generates random number
    let secret_number= rand::thread_rng().gen_range(1,10);

    //Loop so user can guess until they find the correct number
    loop{
        println!("Guess: ");
        //Instailizes guess variable
        let mut guess = String::new();
        //Gets user input
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //Makes guess an integer
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("Secret number: {}", secret_number);
        //Match statement to check if you guessed to high or too low
        match guess.cmp(&secret_number){
            Ordering::Less=> println!("Too small"),
            Ordering::Greater=> println!("Too big"),
            Ordering::Equal=> {
                println!("Correct");
                break;
            }
        }
    }
}
