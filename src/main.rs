
extern crate serde;
extern crate egg_mode;
extern crate rustbreak;
extern crate tokio;
extern crate regex;

mod models;
mod db;
mod display;

use std::error;

const CONSUMER_KEY: &'static str = "***REMOVED***";
const CONSUMER_SEGRET_KEY: &'static str = "***REMOVED***";

use models::{tweet, user, search_terms};

async fn generate_twitter_credentials(user: &mut user::User) -> Result<(), Box<dyn error::Error>> {
    let consumer_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SEGRET_KEY);
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
async fn main() -> Result<(), Box<dyn error::Error>> {
    let search_terms = search_terms::load_terms()?;

    let mut current_user = user::User::new()?;

    if current_user.token.is_none() {
        generate_twitter_credentials(&mut current_user).await?;
        current_user.save()?;
    }

    if let Some(token) = current_user.token {
        tweet::search(&token, String::from(search_terms.join(" "))).await?;
    }

    tweet::processing().await?;

    display::resume_display()?;

    Ok(())
}
