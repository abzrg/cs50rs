fn main() {
    let height = get_positive_integer("Height: ");
    println!("You entered: {}", height);
    mario_more_comfortable(height);
}

#[allow(dead_code)]
fn mario_less_comfortable(height: usize) {
    for row in 0..height {
        let blanks = height - row - 1;
        let bricks = row + 1;

        for _ in 0..blanks {
            print!(" ");
        }

        for _ in 0..bricks {
            print!("#")
        }

        println!();
    }
}

#[allow(dead_code)]
fn mario_more_comfortable(height: usize) {
    let gap = "  ";

    for row in 0..height {
        let blanks = height - row - 1;
        let bricks = row + 1;

        // .repeat(): creates a new String by repeating it n times
        //
        // impl Add<&str> for String:
        // You can add a &str (string slice) to a String, but not another String directly.
        let left = " ".repeat(blanks) + &"#".repeat(bricks);
        // let left = format!("{}{}", " ".repeat(blanks), "#".repeat(bricks));
        let right = "#".repeat(bricks);

        println!("{left}{gap}{right}");
    }
}

fn get_positive_integer(prompt: &str) -> usize {
    // `std::io::self` Brings the whole io module into scope (`io::stdin()`, etc.).
    // `std::io::Write` Imports the `Write` trait so you can call `.flush()` on `stdout`.
    //
    // To call `.flush()` on `stdout` (as in `io::stdout().flush()`), Rust needs the `Write` trait
    // *in scope*.
    use std::io::{self, Write};

    loop {
        print!("{}", prompt);
        // Make sure the prompt is printed
        //
        // When you write with print!, Rust *buffers* output (it may not show up immediately).
        // `.flush()` forces it to appear right away.
        // This makes sure the prompt is visible before waiting for input.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Error handling with pattern matching
        //
        // .parse() returns a `Result`
        match input.trim().parse::<usize>() {
            Ok(n) if n > 0 => return n,
            _ => println!("Please enter a positive (non-zero and non-negative) integer."),
        }
    }
}
