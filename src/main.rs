use std::io::{self, Write};

use clap::Parser;

const APOD: &str = "https://apod.nasa.gov/apod/";
const QT: &str = "\"";
const IMG_PAT: &str = "IMG SRC=\"";

/// Downloads and writes the Astronomy Picture of the Day to standard output
#[derive(Parser)]
#[clap(author, version)]
struct Cli {}

fn get_image_url(html: &str) -> Option<String> {
    if !html.contains(IMG_PAT) {
        return None;
    }

    let img_uri = html
        .split(IMG_PAT)
        .last()
        .and_then(|segment| segment.split(QT).next())
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

#[cfg(test)]
mod tests {
    use crate::get_image_url;

    #[test]
    fn it_extracts_the_url() {
        let html = "<IMG SRC=\"images/image.png\" />";
        let result = get_image_url(html);

        assert_eq!(
            result,
            Some(format!("https://apod.nasa.gov/apod/images/image.png"))
        );
    }

    #[test]
    fn it_returns_none_when_image_is_not_there() {
        let html = "";
        let result = get_image_url(html);

        assert_eq!(result, None);
    }
}
