use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Go On Guess!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input Now");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failure");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Guess: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }
}
