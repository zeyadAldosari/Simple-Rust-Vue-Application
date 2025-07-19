use std::io::{stdin};
use backend::db::{establish_connection, create_user};

fn main() {
    let connection = &mut establish_connection();

    let mut username = String::new();
    let mut password = String::new();

    println!("Enter your email:");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    println!("\nEnter your password:");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();

    let user = create_user(connection, username, password);
    println!("\nSaved draft {username} with password {password} and id {} . note: user created at {}", user.id, user.created_at);
}
