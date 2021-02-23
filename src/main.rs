
extern crate serde;
extern crate serde_yaml;
extern crate egg_mode;
extern crate rustbreak;
extern crate tokio;
extern crate regex;

mod models;
mod config;
mod db;
mod display;
mod error;

use std::{error::Error};

use models::{tweet, user};
use config::{Config};

async fn generate_twitter_credentials(config: Config, user: &mut user::User) -> Result<(), Box<dyn Error>> {
    let consumer_token = egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret_key);
    let request_token = egg_mode::auth::request_token(&consumer_token, "oob").await?;
    let auth_url = egg_mode::auth::authorize_url(&request_token);

    let mut pin_code = String::new();
    display::get_pin_code_display(auth_url, &mut pin_code);
    
    let (token, user_id, username) = egg_mode::auth::access_token(consumer_token, &request_token, pin_code).await?;

    user.id = Some(user_id);
    user.username = Some(username);
    user.token = Some(token);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let mut current_user = user::User::new()?;

    match current_user.token().await {
        Ok(token) => {
            tweet::search(config.clone(), &token, String::from(config.search_terms.join(" "))).await?;
        },
        Err(_e) => {
            generate_twitter_credentials(config.clone(), &mut current_user).await?;
            current_user.save()?;
        },
    }

    tweet::processing().await?;

    display::resume_display()?;

    Ok(())
}
