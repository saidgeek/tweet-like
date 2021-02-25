extern crate dotenv;
use std::{env, fs, path::Path};

fn main() {
    dotenv::dotenv().ok();

    let twitter_consumer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap();
    let twitter_consumer_key = env::var("TWITTER_CONSUMER_KEY").unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("secrets.rs");

    let contents = format!(
        "
    static TWITTER_CONSUMER_KEY: &'static str = \"{}\";
    static TWITTER_CONSUMER_SECRET: &'static str = \"{}\";
    ",
        twitter_consumer_key, twitter_consumer_secret
    );

    fs::write(&dest_path, contents).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
