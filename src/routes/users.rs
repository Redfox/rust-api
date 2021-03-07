use crate::db;
use crate::errors::{ Errors };

use rocket_contrib::json::{ Json, JsonValue  };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
  user: NewUserData,
}

#[derive(Deserialize)]
struct NewUserData {
  username: String,
  email: String,
  password: String,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>, conn: db::Conn) -> Result<JsonValue, Errors> {
  let new_user = new_user.into_inner().user;

  let secret = String::from("8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=");

  db::users::create(&conn, &new_user.username, &new_user.email, &new_user.password)
      .map(|user| json!({ "user": user.to_user_auth(secret.as_bytes()) }))
      .map_err(|error| {
        let field = match error {
          db::users::UserCreatingError::DuplicatedEmail => "email",
          db::users::UserCreatingError::DuplicatedUsername => "username",
        };
        Errors::new(&[(field, "hasd already been taken")])
      })
}
