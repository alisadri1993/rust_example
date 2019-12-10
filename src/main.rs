use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let mut temp = String::new();

    // io::stdin().read_line(&mut guess)
    //     .ok()
    //     .expect("Failed to read line");
    io::stdin().read_line(&mut temp)
    .ok()
    .expect("fail");

    println!("You guessed: {}", temp);
}
