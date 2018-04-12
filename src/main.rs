use std::{io, io::Write};

fn get_user_input_number() -> Result<u32, io::Error> {
    print!("\n> ");

    io::stdout().flush().unwrap();

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

fn show_welcome_message() {
    println!("Welcome to Intencja Game!");

    println!(
        "Press 1 to start a new game.\n\
         Press 2 to enter game settings.\n\
         Press 3 to read game tutorial\n\
         Press 4 to exit game."
    );
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
    show_welcome_message();

    let chosen_option = get_user_input_number().expect("Error getting user input!");

    process_option(chosen_option);
}
