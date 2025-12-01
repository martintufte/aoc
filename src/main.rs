use std::io::stdin;

fn main() {
    println!("Please input your name.");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You name is {}", guess);
}
