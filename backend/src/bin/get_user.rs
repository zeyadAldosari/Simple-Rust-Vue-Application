use backend::models::*;
use diesel::prelude::*;
use backend::{db::establish_connection};
use backend::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::*;

    let user_id = args()
        .nth(1)
        .expect("get_user requires a user id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let user = users
        .find(user_id)
        .select(User::as_select())
        .first(connection)
        .optional();

    match user {
        Ok(Some(user)) => println!("User with id: {} has a username: {}", user.id, user.username),
        Ok(None) => println!("Unable to find user {}", user_id),
        Err(_) => println!("An error occured while fetching user {}", user_id),
    }
}
