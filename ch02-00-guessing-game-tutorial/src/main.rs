use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..51);

    // Loop until the guess is made
    loop {
        println!("Input you guess:");

        // Create a mutable variable with String type
        let mut guess = String::new();

        io::stdin()
            // Gives read_line() a mutable reference to write to
            .read_line(&mut guess)
            // read_line() returns a Result type, which is an enumerator of the types
            // Ok and Err. If everything goes well, then Ok() is returned, else it's
            // Err().
            // If the Result is an Err, then expect() will crash with the string it
            // contains and the Err's value.
            .expect("Failed to read the input");

        // Trim the input, and parse it to a u32 (parse() being aware of the u32 type being expected)
        // Use match instead of expect to catch the Err and go to the next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("It's bigger"),
            Ordering::Greater => println!("It's smaller"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
