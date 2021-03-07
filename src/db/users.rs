use crate::models::user::User;
use crate::schema::users;
use crypto::scrypt::{ scrypt_check, scrypt_simple, ScryptParams };
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub email: &'a str,
  pub hash: &'a str,
}

pub enum UserCreatingError {
  DuplicatedEmail,
  DuplicatedUsername,
}

impl From<Error> for UserCreatingError {
  fn from(err: Error) -> UserCreatingError {
    if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
      match info.constraint_name() {
        Some("users_username_key") => return UserCreatingError::DuplicatedUsername,
        Some("users_email_key") => return UserCreatingError::DuplicatedEmail,
        _ => {}
      }
    }
    panic!("Error creating user: {:?}", err);
  }
}

pub fn create(
  conn: &PgConnection,
  username: &str,
  email: &str,
  password: &str,  
) -> Result<User, UserCreatingError> {
  let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");
  
  let new_user = &NewUser {
    username,
    email,
    hash,
  };

  diesel::insert_into(users::table)
      .values(new_user)
      .get_result::<User>(conn)
      .map_err(Into::into)
}
