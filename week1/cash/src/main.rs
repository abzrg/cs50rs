use std::io::{self, Write};

fn main() {
    const COINS: [u32; 4] = [25, 10, 5, 1];

    print!("Change owed: ");
    io::stdout().flush().unwrap();

    let mut input = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut change: u32 = input
        .trim()
        .parse()
        .expect("Invalid input. Please provide a positive number for the amount of change.");

    let mut number_of_coins = 0;
    for coin in COINS {
        if change == 0 {
            break;
        }

        let quotient = change / coin;
        if quotient != 0 {
            change -= quotient * coin;
            number_of_coins += quotient;
        }
    }

    println!("{number_of_coins}");
}
