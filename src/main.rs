extern crate egg_mode;
extern crate regex;
extern crate rustbreak;
extern crate serde;
extern crate serde_yaml;
extern crate tokio;

mod config;
mod db;
mod display;
mod error;
mod models;

use std::{error::Error, io::Read};

use config::Config;
use models::{tweet, user};

async fn generate_twitter_credentials(
    config: Config,
    user: &mut user::User,
) -> Result<(), Box<dyn Error>> {
    let consumer_token = egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret_key);
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

use std::io::{stdin, stdout, Write};

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let mut current_user = user::User::new()?;

    match current_user.token().await {
        Ok(token) => {
            tweet::search(
                config.clone(),
                &token,
                String::from(config.search_terms.join(" ")),
            )
            .await?;
        }
        Err(_e) => {
            generate_twitter_credentials(config.clone(), &mut current_user).await?;
            current_user.save()?;
        }
    }

    tweet::processing().await?;

    display::resume_display()?;

    pause();

    Ok(())
}
