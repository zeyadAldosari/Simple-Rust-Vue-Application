use diesel::prelude::*;
use backend::{db::establish_connection};
use backend::*;
use std::env::args;

fn main() {
    use schema::users::dsl::*;

    let user_id = args()
        .nth(1)
        .expect("get_user requires a user id")
        .parse::<i32>()
        .expect("Invalid ID");
  

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(users.find(user_id))
        .execute(connection)
        .expect("Error deleting user");

    println!("Deleted {} user", num_deleted);
}
