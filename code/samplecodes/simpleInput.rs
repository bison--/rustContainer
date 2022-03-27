
fn main() {
    let user_name = get_user_input("Enter name:");
    println!("Your Name: {}", &user_name);

    let user_age: i8 = get_user_input("Enter your age:").parse().unwrap();
    println!("Your Age: {}", user_age);
}

fn get_user_input(question: &str) -> std::string::String {
    use std::io::{stdin, stdout, Write};

    let mut user_input = String::new();
    print!("{}", question);

    let _ = stdout().flush();
    stdin().read_line(&mut user_input).expect("Did not enter a correct string");

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }

    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    return user_input;
}
