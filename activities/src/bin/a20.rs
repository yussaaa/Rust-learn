// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn print_power_status(power_state: PowerState) {
    match power_state {
        PowerState::Off => println!("Powering off"),
        PowerState::Sleep => println!("Sleeping"),
        PowerState::Reboot => println!("Rebooting"),
        PowerState::Shutdown => println!("Shutting down"),
        PowerState::Hibernate => println!("Hibernating"),
    }
}

use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a power state: ");
    io::stdin().read_line(&mut input);
    let power_state = match input.trim().to_lowercase().as_str() {
        "off" => PowerState::Off,
        "sleep" => PowerState::Sleep,
        "reboot" => PowerState::Reboot,
        "shutdown" => PowerState::Shutdown,
        "hibernate" => PowerState::Hibernate,
        _ => panic!("Invalid power state"),
    };
    print_power_status(power_state);
}
