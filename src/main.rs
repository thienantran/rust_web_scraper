use reqwest::blocking::Client;
use scraper::{Html, Selector, ElementRef};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://blog.google/technology/ai/google-gemini-next-generation-model-february-2024/";
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; YourBot/0.1; +http://yourbot.com/info)")
        .build()?;

    let resp = client.get(url).send()?.text()?;
    let fragment = Html::parse_document(&resp);

    let selectors = ["p", "h1", "h2", "img", "a"];
    let parsed_selectors = selectors.iter()
        .map(|&sel| {
            Selector::parse(sel).unwrap_or_else(|_| panic!("Failed to parse selector: {}", sel))
        })
        .collect::<Vec<Selector>>();

    for (i, selector) in parsed_selectors.iter().enumerate() {
        for element in fragment.select(selector) {
            let sel_str = selectors[i]; // Use the original selector string for matching
            match sel_str {
                "img" => {
                    if let Some(src) = element.value().attr("src") {
                        println!("Image URL: {}", resolve_url(url, src));
                    }
                },
                "a" => {
                    if let Some(href) = element.value().attr("href") {
                        println!("Link URL: {}", resolve_url(url, href));
                    }
                },
                _ => {
                    let text = element.text().collect::<Vec<_>>().join(" ");
                    println!("{}: {}", sel_str, text); // Use sel_str directly
                },
            }
        }
    }

    Ok(())
}

fn resolve_url(base_url: &str, url: &str) -> String {
    // Placeholder for URL resolution logic
    let parsed_base_url = url::Url::parse(base_url).expect("Failed to parse base URL");
    parsed_base_url.join(url).unwrap_or_else(|_| parsed_base_url.clone()).to_string()
}

