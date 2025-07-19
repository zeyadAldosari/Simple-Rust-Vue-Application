use backend::models::*;
use diesel::prelude::*;
use backend::{db::establish_connection};
use backend::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    println!("-----------");
    for user in results {
        println!("{}", user.username);
        println!("{}", user.password);
        println!("-----------");

    }
}
