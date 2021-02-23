use std::fs::File;
use std::env;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub type SearchTerms = Vec<String>;

pub fn load_terms() -> io::Result<SearchTerms> {
  let mut terms: SearchTerms = Vec::new();

  let mut path = PathBuf::new();
  path.push(env::current_dir()?);
  path.push("search-terms.txt");

  let file = File::open(path)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line.unwrap();

    terms.push(line.to_lowercase().trim().to_string());
  }

  Ok(terms)
}