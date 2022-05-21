use std::io::{self, Write};

use clap::Parser;

const APOD: &str = "https://apod.nasa.gov/apod/";
const IMG_PAT: &str = "IMG SRC=\"";

/// Astronomy Picture of the Day in your terminal
#[derive(Parser)]
struct Cli {}

fn get_image_url(html: &str) -> Option<String> {
    if !html.contains(IMG_PAT) {
        return None;
    }

    let img_uri = html
        .split(IMG_PAT)
        .last()
        .and_then(|segment| segment.split("\"").next())
        .unwrap();

    Some(format!("{}{}", APOD, img_uri))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();

    let apod_html_resp = reqwest::blocking::get(APOD)?.text()?;

    match get_image_url(&apod_html_resp) {
        Some(image_url) => {
            let buffer = reqwest::blocking::get(&image_url)?.bytes()?;
            io::stdout().write_all(&buffer)?;

            Ok(())
        }
        None => panic!("could not find the picture"),
    }
}
