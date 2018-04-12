use std::{io, io::Write};

fn show_welcome_message() -> Result<u32, std::io::Error> {
    println!("Welcome to Intencja Game!");

    print!(
        "Press 1 to start a new game.\n\
         Press 2 to enter game settings.\n\
         Press 3 to read game tutorial\n\
         Press 4 to exit game.\n>"
    );

    io::stdout().flush();

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input!");

    let user_input: u32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse input. You must enter a number!");

    Ok(user_input)
}

fn start_new_game() {
    println!("Starting new game!");
}

fn show_settings() {
    println!("Game settings:");
}

fn show_tutorial() {
    println!("How to play Intencja:");
}

fn exit_game() {
    println!("Exiting game!");
}

fn process_option(chosen_option: u32) {
    match chosen_option {
        1 => start_new_game(),
        2 => show_settings(),
        3 => show_tutorial(),
        4 => exit_game(),
        _ => println!("Wrong option"),
    }
}

fn main() {
    let chosen_option = show_welcome_message().expect("Error getting user input!");

    process_option(chosen_option);
}
