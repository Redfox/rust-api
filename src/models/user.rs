use crate::config::token;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  #[serde(skip_serializing)]
  pub hash: String
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
  username: &'a str,
  email: &'a str,
  token: String
}

#[derive(Serialize)]
pub struct Profile {
  username: String,
}

impl User {
  pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
    let user = token::UserInfo {
      id: self.id,
      username: self.username.clone(),
    };

    let token = token::generate_token(user, secret);

    UserAuth {
      username: &self.username,
      email: &self.email,
      token
    }
  }

  pub fn to_profile(self) -> Profile {
    Profile {
      username: self.username,
    }
  }
}
