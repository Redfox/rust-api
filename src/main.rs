#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/<name>")]
fn index(name: String) -> Json<Value> {
  
  Json(json!(format!("hello, {}", name)))
}  

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .launch();
}
