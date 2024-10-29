use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("guess less than secret_num"),
            Ordering::Greater => println!("guess greater than secret_num"),
            Ordering::Equal => {
                println!("GOTCHA");
                break;
            }
        }
    }
}
