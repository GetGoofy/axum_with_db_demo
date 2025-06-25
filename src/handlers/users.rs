use axum::Json;

use crate::{database::establish_connection, models::{NewUser, User}, operations::users::create_user};


pub async fn create_user_handler(Json(payload): Json<NewUser>) -> Json<User> {
    let db_connection = &mut establish_connection();
    let created_user = create_user(db_connection, &payload.email, &payload.name).await;
    Json(created_user)
}