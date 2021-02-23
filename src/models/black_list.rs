use std::fs::File;
use std::env;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub type BlackList = Vec<String>;

pub fn load_black_list() -> io::Result<BlackList> {
  let mut list: BlackList = Vec::new();

  let mut path = PathBuf::new();
  path.push(env::current_dir()?);
  path.push("black_list.txt");

  let file = File::open(path)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line.unwrap();

    list.push(line.to_lowercase().trim().to_string());
  }

  Ok(list)
}