use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::{models::{NewUser, User}, schema::users};


pub async fn create_user(conn: &mut PgConnection, email: &str, name: &str) -> User {
    let new_email = email.to_string();
    let new_name = name.to_string();
    let new_user = NewUser {email: new_email, name: new_name};

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error creating new user")
}