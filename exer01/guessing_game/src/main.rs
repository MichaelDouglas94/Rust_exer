use std::io;
use rand::Rng; // método para gerar números aleatórios


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess!");

    // Comentários, o mut permite que a variável seja mutável. 
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guesssed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::less => println!("Too small!"),
        Ordering::Greater => println("Too big"),
        Ordering::Equal => println!("You win!!"),
    }
}

// arm’s pattern. tradução 