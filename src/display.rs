use std::error;
use crate::models::{tweet};

pub fn resume_display() -> Result<(), Box<dyn error::Error>>{
  println!("Pending tweets: {}", tweet::get_pending()?.len());
  println!("Liked tweets: {}", tweet::get_liked()?.len());
  println!("Discarted tweets: {}", tweet::get_discarted()?.len());
  println!("Total tweets: {}", tweet::get_all()?.len());

  Ok(())
}

pub fn get_pin_code_display(url: String, pin_code: &mut String) {
  println!("Go to the following URL, sign in, and give me the PIN that comes back:");
  println!("{}", url);

  std::io::stdin().read_line(pin_code).unwrap();
  println!("");
}