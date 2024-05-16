use std::io;

fn main() {
    let mut firstname = String::new();
    let mut lastname = String::new();

    // get first name input
    println!("Your first name:");
    io::stdin()
        .read_line(&mut firstname)
        .expect("Failed to read line");

    // get last name input
    println!("Your last name:");
    io::stdin()
        .read_line(&mut lastname)
        .expect("Failed to read line");

    // trim strings
    let firstname = firstname.trim();
    let lastname = lastname.trim();

    println!("Your legal name is {firstname} {lastname}.");
}
