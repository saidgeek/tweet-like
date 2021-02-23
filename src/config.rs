use std::fs;
use std::env;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
  pub consumer_key: String,
  pub consumer_secret_key: String,
  pub search_count: u32,
  pub search_terms: Vec<String>,
  pub black_list: Vec<String>,
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {

  let mut path = PathBuf::new();
  path.push(env::current_dir()?);
  path.push("config.yaml");

  let content = fs::read_to_string(path)?.parse::<String>()?;
  let config= serde_yaml::from_str::<Config>(content.as_str())?;

  Ok(config)
}
