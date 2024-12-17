use std::io;

fn main() {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice);

    let choice = choice.trim();

    for letter in choice.to_lowercase().chars() {
        if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' ||  letter == 'u' {
            println!("Your word: {choice}-hay");
            break;
        }
        else {
            let real_choice = &choice[1..];
            println!("Your word: {real_choice}-{letter}ay");
            break;
        }
    }
}

