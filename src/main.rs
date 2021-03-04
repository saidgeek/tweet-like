extern crate egg_mode;
extern crate regex;
extern crate rustbreak;
extern crate serde;
extern crate serde_yaml;
extern crate tokio;
extern crate trim_margin;
#[macro_use] extern crate log;

mod db;
mod display;
mod error;
mod models;
#[cfg(target_os = "windows")]
mod pause;

use std::{error::Error};
use models::{tweet, user};
use log::LevelFilter;
use env_logger::Builder;
use crate::models::settings::Settings;


include!(concat!(env!("OUT_DIR"), "/secrets.rs"));

async fn generate_twitter_credentials(user: &mut user::User) -> Result<(), Box<dyn Error>> {
    let consumer_token = egg_mode::KeyPair::new(TWITTER_CONSUMER_KEY, TWITTER_CONSUMER_SECRET);
    let request_token = egg_mode::auth::request_token(&consumer_token, "oob").await?;
    let auth_url = egg_mode::auth::authorize_url(&request_token);

    let mut pin_code = String::new();
    display::get_pin_code_display(auth_url, &mut pin_code);

    let (token, user_id, username) =
        egg_mode::auth::access_token(consumer_token, &request_token, pin_code).await?;

    user.id = Some(user_id);
    user.username = Some(username);
    user.token = Some(token);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::load()?;
    let mut log_builder = Builder::new();

    log_builder
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .format_module_path(false);

    let _ = log_builder.try_init();

    let mut current_user = user::User::new()?;

    info!("Verified if exists some user initialized.");
    match current_user.token().await {
        Ok(token) => {
            info!("Search tweets...");
            tweet::search(&token).await?;
        }
        Err(_e) => {
            info!("Please login user!");
            generate_twitter_credentials(&mut current_user).await?;
            current_user.save()?;

            let token = current_user.token().await?;
            info!("Search tweets...");
            tweet::search(&token).await?;
        }
    }

    if settings.is_missing() {
        warn!("Please edit a settings file for input the search terms and blacklist.");
    } else {
        info!("Processing the found tweets");
        tweet::processing().await?;
    
        info!("Process finished:");
        display::resume_display()?;
    }
    
    #[cfg(target_os = "windows")]
    pause::pause();

    Ok(())
}
