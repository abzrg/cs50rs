use std::io::{self, Write};

fn main() {
    // `const`: Compile time constants; similar to constexpr in C++
    // They should be in UPPER_SNAKE_CASE
    const COINS: [u32; 4] = [25, 10, 5, 1];

    let change = read_validated_change("Change owed: ");

    // `.iter()` returns an iterator (Iter) over an slice of coin values
    // The iterator yields all the items from start to end, i.e., it gives you *references* to the
    // values: &25, &10, etc.
    //
    // `fold()` folds every element into an accumulator by applying an operation, returning the
    // final result.
    // `fold()` takes two arguments:
    // - an initial value,
    // - a closure with two arguments:
    //   - an 'accumulator',
    //   - an element.
    // The closure returns the value that the accumulator should have for the next iteration.
    // The initial value is the value the accumulator will have on the first call.
    // After applying this closure to every element of the iterator, `fold()` returns the
    // accumulator.
    // This operation is sometimes called 'reduce' or 'inject'.
    // Folding is useful whenever you have a collection of something, and want to produce a single
    // value from it.
    //
    // Here it starts with an initial value of the tuple: (0, cents)
    // - `0`: coint count
    // - `cents`: the change amount
    // Each iteration receives:
    // - (`count`,`remaining`): the total number of coins so far and how much change is still left.
    // - `coin`: the value of the current coin (by dereferencing the `&coin`).
    // Each iteration returns a new (count, remaining) pair, passed to the next one.
    // Finally, fold returns a tuple: `(count, remaining)`
    // `.0` gets the first item — the final coin count.
    //
    // Why use `fold`?
    // - No mutation (let mut ...)
    // - Expresses "reduce a list to a value" clearly
    // - Clean, functional, composable style
    let coin_count = COINS
        .iter()
        .fold((0, change), |(count, remaining), &coin| {
            // How many times the current coin fits into remaining.
            let num = remaining / coin;
            (count + num, remaining - num * coin)
        })
        .0;

    println!("{coin_count}");
}

fn get_int() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        // `.map_err()` maps a `Result<T, E>` to `Result<T, F>` by applying a function to a
        // contained `Err` value, leaving an `Ok` value untouched. This function can be used to
        // pass through a successful result while handling an error.
        //
        // Here:
        // - `read_line` returns `Result<usize, std::io::Error>`
        // - `map_err` converts the error type to `String`
        // - The result becomes `Result<usize, String>`
        // So if reading from stdin fails, we get a human-readable error message like:
        //   "Read Error: <description>"
        //
        // The `?` operator is shorthand for error propagation.
        // It:
        // - Returns the `Ok` value if the result is `Ok`
        // - Otherwise, returns early from the function with the `Err` value
        //
        // Here:
        // - If reading from stdin succeeds, continue.
        // - If it fails, return `Err(String)`, ending the function early.
        .map_err(|e| format!("Read Error: {}", e))?;
    input
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("Parse Error: {}", e))
}

fn read_validated_change(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        match get_int() {
            Ok(cents) if cents > 0 => {
                return cents as u32;
            }
            Ok(_) => {
                eprintln!("Please enter a positive, non-zero amount.");
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
