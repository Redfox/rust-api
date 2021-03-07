#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod routes;

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
  rocket::ignite()
    .mount("/api",
    routes![
      routes::users::post_users,
    ],
  )
  .register(catchers![not_found, unprocessable_entity])
}
