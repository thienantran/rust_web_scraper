use reqwest;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL to scrape
    let url = "https://www.nbcnews.com/politics/congress/failure-republicans-will-try-impeach-mayorkas-border-rcna138474ca";

    // Create a Client instance with a custom User-Agent
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()?;

    // Make a synchronous GET request using the client
    let resp = client.get(url).send()?.text()?;

    // Parse the HTML
    let fragment = Html::parse_document(&resp);

    // Create a Selector for parsing
    let stories_selector = Selector::parse("p").unwrap(); // Change "p" to the appropriate CSS selector for the content you want

    // Iterate through the selected elements
    for story in fragment.select(&stories_selector) {
        let story_txt = story.text().collect::<Vec<_>>().join(" ");
        println!("{}", story_txt);
    }

    Ok(())
}
