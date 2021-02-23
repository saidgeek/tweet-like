use std::{
  error,
  result::Result
};
use serde::{Deserialize, Serialize};
use egg_mode;
use crate::db;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Option<u64>,
  pub username: Option<String>,
  pub token: Option<egg_mode::Token>,
}

impl User {
  pub fn new() -> Result<User, Box<dyn error::Error>> {
    let db = db::init_db()?;

    let user = db.read(|db| {
      db.user.clone()
    })?;

    if !user.token.is_none() {
      return Ok(user);
    }

    Ok(User {
      id: None,
      username: None,
      token: None,
    })
  }

  pub fn save(&mut self) -> Result<(), Box<dyn error::Error>> {
    let db = db::init_db()?;

    db.write(|db| db.user = self.clone())?;
    db.save()?;

    Ok(())
  }
}