use std::io;

enum UserResponse {
    L,
    H,
    A,
}

fn prompt(guess: i32, can_be_lower: bool, can_be_higher: bool) -> UserResponse {
    let mut question = String::new();
    let response: UserResponse;

    println!("My guess is: {}", guess);

    question.push_str("Is it ");
    if can_be_lower {
        question.push_str("[L]ower");
    }
    if can_be_lower && can_be_higher {
        question.push_str(", ");
    }
    if can_be_higher {
        question.push_str("[H]igher");
    }
    question.push_str(" or the [A]nswer?");

    loop {
        let mut input = String::new();

        println!("{}", question);

        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        let selection = input[0..1].to_uppercase();

        match selection.as_str() {
            "L" => {
                if can_be_lower {
                    break response = UserResponse::L;
                } else {
                    println!("It can't be lower. Try again...");
                }
            }
            "H" => {
                if can_be_higher {
                    break response = UserResponse::H;
                } else {
                    println!("It can't be higher. Try again..");
                }
            }
            "A" => {
                break response = UserResponse::A;
            }
            _ => {
                println!("Please respond with L, H or A...");
            }
        }
    }

    response
}

fn main() {
    let min = 1;
    let max = 99;

    let mut min_guess = min;
    let mut max_guess = max;
    let mut answer = 0;
    let mut guess;
    let mut tries = 0;

    println!("Welcome to Guesser-rs. The Rust version of my favorite programming test.");
    println!();
    println!(
        "Think of a number between {} and {} (including) and answer my questions:",
        min_guess, max_guess
    );
    println!();

    loop {
        guess = (min_guess + max_guess) / 2;

        let can_be_lower = guess > min_guess;
        let can_be_higher = guess < max_guess;

        let response = prompt(guess, can_be_lower, can_be_higher);

        match response {
            UserResponse::L => {
                if guess > min_guess {
                    max_guess = guess - 1;
                }
                tries += 1;
            }
            UserResponse::H => {
                if guess < max_guess {
                    min_guess = guess + 1;
                }
                tries += 1;
            }
            UserResponse::A => {
                answer = guess;
            }
        }

        if min_guess == max_guess {
            answer = min_guess;
        }

        if answer > 0 {
            println!("The answer is {}! I guessed it in {} tries!", answer, tries);
            break;
        }
    }
}
