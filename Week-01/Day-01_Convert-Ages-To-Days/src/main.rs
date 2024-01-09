use std::io;
use anyhow::Result;
const DAYS_IN_YEAR: u32 = 365;

fn main() -> Result<()> {
    // Messages to User
    println!("Welcome to our Year to Day age converter!");
    println!("Please input your age in years!");

    // Create a empty string to handle the user input
    let mut guess = String::new();

    // Populate string using stdin
    io::stdin().
        read_line(&mut guess)
        .expect("Could not read age. Make sure an integer is used.");

    // Convert guess to the age in days
    match convert_years_to_days(guess.trim()) {
        Ok(age) =>  println!("Silly rabbit you are {} days old not {} !", age, guess.trim_end()),
        Err(e) => println!("Error converting string:, {}", e),
    }
    Ok(())
}

//  Simple function to convert year to days as string/str
fn convert_years_to_days(year: &str) -> Result<String> {
    // Converts string to u32
    let age_in_days: u32 = year.parse()?;

    // if no errors multiplies it by the days in a year constant and converts it back to a string back returns the valve
    Ok((age_in_days * DAYS_IN_YEAR).to_string())
}
