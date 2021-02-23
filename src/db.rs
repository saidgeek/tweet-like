use std::collections::HashMap;

use rustbreak::FileDatabase;
use rustbreak::deser::Ron;
use serde::{Deserialize, Serialize};
use crate::models::{tweet, user, search_terms, black_list};

pub type DB = FileDatabase<Storage, Ron>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Storage {
  pub user: user::User,
  pub search_terms: search_terms::SearchTerms,
  pub black_list: black_list::BlackList,
  pub tweets: HashMap<u64, tweet::Tweet>,
}

pub fn init_db() -> Result<DB, Box<dyn std::error::Error>> {
  let db: DB = FileDatabase::create_at_path("storage.ron", Storage {
    user: user::User {
      id: None,
      username: None,
      token: None,
    },
    search_terms: Vec::new(),
    black_list: Vec::new(),
    tweets: HashMap::new(),
  })?;

  let _ = db.load()?;

  Ok(db)
}