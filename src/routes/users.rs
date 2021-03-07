use rocket_contrib::json::{ Json, JsonValue  };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize)]
struct NewUserData {
  username: Option<String>,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>) -> JsonValue {
  
  let new_user = new_user.into_inner().user;

  json!({
    "username": new_user.username,
  })
}
