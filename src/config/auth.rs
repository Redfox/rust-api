use serde::{ Deserialize, Serialize };

use jsonwebtoken as jwt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
  pub exp: i64,
  pub id: i32,
  pub username: String,
}

impl Auth {
  pub fn token(&self, secret: &[u8]) -> String {
    jwt::encode(&jwt::Header::default(), self, secret).expect("jwt")
  }
}
