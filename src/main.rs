use error_chain::error_chain;
use std::{thread, time};
use chrono::{DateTime, Utc};
use std::io::{self, Write};
use std::io::prelude::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    loop {
        let res = reqwest::blocking::get("https://rss.globenewswire.com/HexMLItem/Content/FullText/Attachments/All/Identifier/1007942/language/en")?;
        if res.status().is_success() {
            if let Some(content_length) = res.content_length() {
                match content_length {
                    3530 => {
                        print!(".");
                        io::stdout().flush().unwrap();
                    },
                    _ => {
                        let now: DateTime<Utc> = Utc::now();
                        println!("Time now is: {}", now.to_rfc3339());
                        println!("\nContent-length: {}", content_length);
                        println!("Body:\n{}", res.text().unwrap());
                        let mut input = io::stdin();
                        let _ = input.read(&mut [0u8]).unwrap();
                        return Ok(());
                    }
                }
            }
        }
        else {
              print!("x");
              io::stdout().flush().unwrap();
              return Ok(());
        }
        thread::sleep(time::Duration::from_secs(60));
    }
}
