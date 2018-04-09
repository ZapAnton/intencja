fn show_welcome_message() -> Result<i32, std::io::Error> {
    println!("Welcome to Intencja Game!");

    print!(
        "Press 1 to start a new game.\n\
        Press 2 to enter game settings.\n\
        Press 3 to read game tutorial\n\
        Press 4 to exit game.\n>"
    );

    //TODO Get user input
    
    Ok(1)
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

fn process_option(chosen_option: i32) {
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
