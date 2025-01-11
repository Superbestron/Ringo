use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exp = async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    };
    Runtime::new().unwrap().block_on(exp);
}

async fn page_title(url: &str) -> Option<String> {
    let response = reqwest::get(url).await.unwrap();
    let response_text = response.text().await.unwrap();
    let parsed = scraper::Html::parse_document(&response_text);
    let selector = scraper::Selector::parse("title").unwrap();
    let opt = parsed.select(&selector).nth(0);
    opt.map(|title_element| title_element.inner_html())
}
