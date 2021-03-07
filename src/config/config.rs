use rocket::config::{ Value, Config, Environment };
use std::collections::HashMap;
use std::env;

fn config_databases() -> HashMap<&'static str, rocket::config::Value> {
  let mut database_config = HashMap::new();
  let mut databases = HashMap::new();
  let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");

  database_config.insert("url", Value::from(database_url));
  databases.insert("diesel_postgres_pool", Value::from(database_config));

  return databases;
}

pub fn default() -> Config {
  let environment = Environment::active().expect("No environment found");

  let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

  let databases = config_databases();

  Config::build(environment)
      .environment(environment)
      .port(port)
      .extra("databases", databases)
      .finalize()
      .unwrap()
}
