use crate::db;
use crate::error::{self, Error};
use egg_mode;
use serde::{Deserialize, Serialize};

pub type UserResult<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub token: Option<egg_mode::Token>,
}

impl User {
    pub fn new() -> UserResult<User> {
        let db = db::init_db()
            .map_err(|_| Error::User(error::ErrorKind::NotCreate))
            .unwrap();

        let user = db
            .read(|db| db.user.clone())
            .map_err(|_| Error::User(error::ErrorKind::NotCreate))
            .unwrap();

        if !user.token.is_none() {
            return Ok(user);
        }

        Ok(User {
            id: None,
            username: None,
            token: None,
        })
    }

    pub fn save(&mut self) -> UserResult<()> {
        let db = db::init_db()
            .map_err(|_| Error::User(error::ErrorKind::NotSave))
            .unwrap();

        db.write(|db| db.user = self.clone())
            .map_err(|_| Error::User(error::ErrorKind::NotSave))
            .unwrap();
        db.save()
            .map_err(|_| Error::User(error::ErrorKind::NotSave))
            .unwrap();

        Ok(())
    }

    pub async fn token(&self) -> UserResult<egg_mode::Token> {
        if let Some(token) = self.token.clone() {
            if let Err(_) = egg_mode::auth::verify_tokens(&token).await {
                return Err(Error::User(error::ErrorKind::InvalidToken));
            }
            return Ok(token);
        } else {
            return Err(Error::User(error::ErrorKind::Unauthorized));
        }
    }
}
