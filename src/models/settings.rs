use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::PathBuf;
use std::{env::current_dir, error};
use trim_margin::MarginTrimmable;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub search_count: u32,
    pub search_terms: Vec<String>,
    pub black_list: Vec<String>,
}

impl Settings {
    pub fn load() -> Result<Self, Box<dyn error::Error>> {
        let path = current_dir().and_then(|p| Ok(p.join("settings.yaml")))?;

        if !path.exists() {
            create_settings_file(&path)?;
        }

        let content = fs::read_to_string(&path)?.parse::<String>()?;
        let config = serde_yaml::from_str::<Self>(content.as_str())?;

        Ok(config)
    }
}

fn create_settings_file(path: &PathBuf) -> Result<(), Box<dyn error::Error>> {
    let contents = r#"
        |# This field indicate the number of results will.
        |searchCount: 5
        |# This field is a list of the terminus will searching on twitter.
        |searchTerms:
        |    - #rustlang
        |# This field is a black list of terminus will use for discard tweets.
        |blackList:
        |    - lala
    "#;

    fs::write(path, contents.trim_margin().unwrap())?;

    Ok(())
}
