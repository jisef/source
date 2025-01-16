/*mod concepts;

use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {


    let super_secret_number = rand::rng().random_range(1..100);
    println!("Random number : {}" ,super_secret_number);
    
    
    
    let mut input = String::new();

    loop {
        println!("Schreibe was du neger");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("Deine eingabe : {}", input);

        let input: i32 = match input.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue
            },
        }; 
        let input = match input.cmp(&super_secret_number) {
            Ordering::Less    => println!("WOMP WOMP TO SMALL"),
            Ordering::Greater    => println!("WOMP WOMP TO BIG"),
            Ordering::Equal    =>  {
                println!("Correct");
                break;       
            },
        };
    }
    
}
*/
mod concepts;