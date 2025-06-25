use axum::{
    routing::{get, post}, Router
};

use crate::{database::establish_connection, handlers::users, operations::users::create_user};

mod vehicle;
mod database;
mod models;
mod schema;
mod operations;
mod handlers;

#[tokio::main]
async fn main() {
    // ### API example without using DB
    // 1) Create the axum router
    // let router01 = Router::new().route("/", get(vehicle::vehicle_get).post(vehicle::vehicle_post));

    // 2) Define the IP and port listener (TCP) , 0.0.0.0 means accept connections from everywhere
    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // 3) axum serve to launch the webserver
    // axum::serve(listener, router01).await.unwrap();

    // ### API hardcoded example using DB
    // let db_connection = &mut establish_connection();

    // let created_user = create_user(db_connection, "miguel@gmail.com", "miguel").await;
    // println!("Saved user with id: {}", created_user.id);

    // ### API example using DB and JSON input / output
    let router01 = Router::new().route("/", post(users::create_user_handler));

    axum::serve(listener, router01).await.unwrap();
    


}

