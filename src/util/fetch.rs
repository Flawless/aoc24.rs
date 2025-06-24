use reqwest::blocking::Client;
use std::error::Error;

const SESSION_COOKIE: Option<&str> = option_env!("SESSION_COOKIE");

/// Downloads the input data from the Advent of Code website.
fn _input(day: u8, session_cookie: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15")
        .header("Cookie", format!("session={session_cookie}"))
        .send()?;

    if response.status().is_success() {
        let text = response.text()?;
        Ok(text)
    } else {
        Err(format!("Failed to download input: {}", response.status()).into())
    }
}

pub fn input(day: u8) -> Result<String, Box<dyn Error>> {
    match SESSION_COOKIE {
        Some(cookie) => _input(day, cookie),
        None => Err("SESSION_COOKIE environment variable not set".into()),
    }
}
