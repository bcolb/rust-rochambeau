use std::io;
use rand::Rng;

fn main() {
    start_rust_rochambeau();
}

// Simple game driver
fn start_rust_rochambeau(){
    // Using a raw string literal for the intro
    let welcome = r###"
    Welcome to Rust Rochambeau!
    The classic Rock Paper Scissors game implemented in rust-lang
    "###;
    let description = r###"
    This is a simple command line game where you'll be playing against the computer.
    The computer will 'shoot' based off of a random number generator.
    "###;
    let instructions = r###"
    On the command "Rock, Paper, Scissors, Shoot!"
    enter one of the following choices:
    rock
    paper
    scissors
    If you and the computer match you will need to shoot again.
    Otherwise, one of you will win according to these rules:
    - rock beats scissors
    - scissors beats paper
    - paper beats rock
    Enter "exit" to exit the game at any time.
    Enter "help" to see these instructions again.
    "###;
    println!("{}", welcome);
    println!("{}", description);
    println!("{}", instructions);

    // Now let's do a typical loop and command interface
    // This will likely get refactored out
    let mut playing = true;
    let mut cmd = String::new();
    while(playing) {
        // set command to empty string to remove previous input (if any)
        cmd = "".to_string();
        println!("Rock, Paper, Scissors, shoot!");
        io::stdin().read_line(&mut cmd).expect("Failed to read line");
        let mut result = 0;
        match cmd.trim() {
            "exit" => playing = false,
            "rock" | "paper" | "scissors" => result = play_rochambeau_game(cmd.trim()),
            "help" => println!("{}", instructions),
            _ => println!("The command {} was not recognized, type help to see the instructions again or exit to exit the game.", cmd),
        }
    }
}

// Function for a game
fn play_rochambeau_game(user_choice:&str) ->  usize {
    // returns true if user wins, false if user loses
    let num = rand::thread_rng().gen_range(1..4);
    let mut computer_choice = "rock";
    match num {
        1 => computer_choice = "rock",
        2 => computer_choice = "paper",
        3 => computer_choice = "scissors",
        _ => computer_choice = "error",
    }
    println!("User command was: {}", user_choice);
    println!("Computer choice was: {}", computer_choice);

    let mut result = 0; // result states, 0:tie, 1:user wins, 2:computer wins
    // Work through the logic - will refactor later
    if user_choice == computer_choice {
        result = 0;
    } else if user_choice == "rock" && computer_choice == "scissors" {
        result = 1;
    } else if user_choice == "scissors" && computer_choice == "paper" {
        result = 1;
    } else if user_choice == "paper" && computer_choice == "rock" {
        result = 1;
    } else {
        result = 2;
    }

    match result {
        0 => {
            println!("Tie!");
        },
        1 => {
            println!("You win!");
        },
        2 => {
            println!("Computer wins!");
        }
        _ => {
            println!("Erroneous result, replay!");
        }
    }
    return result;
}