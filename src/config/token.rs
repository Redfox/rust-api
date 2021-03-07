use crate::config::auth::Auth;
use chrono::{ Duration, Utc };

pub struct UserInfo {
  pub id: i32,
  pub username: String,
}

pub fn generate_token(user: UserInfo, secret: &[u8]) -> String {
  let exp = Utc::now() + Duration::days(60);
  let token = Auth {
    id: user.id,
    username: user.username.clone(),
    exp: exp.timestamp(),
  }
  .token(secret);

  return token;
}
