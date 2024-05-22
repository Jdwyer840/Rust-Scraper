use fantoccini::{ClientBuilder, Locator};
use scraper::{Html, Selector};

use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

pub async fn let_testing_stuff() -> Result<(), fantoccini::error::CmdError> {
    println!("YO");
    // Connecting using "native" TLS (with feature `native-tls`; on by default)
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");
    // Connecting using Rustls (with feature `rustls-tls`)
    // let c = ClientBuilder::rustls().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");
    // c.wait().on("#mw-content-text .mw-content-ltr.mw-parser-output");

    let results_element = c.find(Locator::Css("#mw-content-text .mw-content-ltr.mw-parser-output")).await?;
    let results_html = results_element.html(false).await?;

    let document = Html::parse_document(&results_html);
    let selector = Selector::parse("p").unwrap();

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join((" "));
        println!("Resuls {text}");
    }

    // click "Foo (disambiguation)"
    c.find(Locator::Css(".mw-disambig")).await?.click().await?;

    // click "Foo Lake"
    // serdexmlcrate
    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    println!("{url}");

    c.close().await
}