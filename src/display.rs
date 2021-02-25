use crate::models::tweet;
use std::io::{Write, stdin, stdout};
use std::error;
use trim_margin::MarginTrimmable;

pub fn resume_display() -> Result<(), Box<dyn error::Error>> {
    let resume = format!(r#"
            |   Resume:
            |       {} Pending.
            |       {} Liked.
            |       {} Discarted.
            |       {} Total.
        "#,
        tweet::get_pending()?.len(),
        tweet::get_liked()?.len(),
        tweet::get_discarted()?.len(),
        tweet::get_all()?.len(),
    );

    println!("\n{}\n", resume.trim_margin().unwrap());

    Ok(())
}

pub fn get_pin_code_display(url: String, pin_code: &mut String) {

    let message  = format!(r#"
        |   Go to the following URL, sign in, and give me the PIN that comes back:
        |   {}
        |   Enter PIN here: 
    "#, url);
    print!("\n{}", message.trim_margin().unwrap());

    let _ = stdout().flush();
    stdin().read_line(pin_code).unwrap();
    println!("");
}
