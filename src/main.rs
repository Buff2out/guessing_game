use rand::Rng;
use std::io;
fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("{secret_num}");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline");
    print!("{guess}");
}
