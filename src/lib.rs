#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod config;
mod routes;
mod db;
mod schema;
mod errors;

use dotenv::dotenv;
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
  json!({
    "status": "error",
    "reason": "Resource was not found."
  })
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
  json!({
    "status": "error",
    "reason": "Invalid data"
  })
}

pub fn rocket() -> rocket::Rocket {
  dotenv().ok();
  rocket::custom(config::config::default())
    .mount("/api",
      routes![
        routes::users::post_users,
      ],
    )
    .attach(db::Conn::fairing())
    .register(catchers![not_found, unprocessable_entity])
}
