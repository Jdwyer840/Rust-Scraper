#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, post, routes, launch, Rocket};
use serde::Deserialize;
use reqwest::Error as ReqwestError;
use scraper::{Html, Selector};
use reqwest::get;
use std::error::Error as StdError;
use std::ffi::c_void;
use std::fmt;
use std::ptr::write;
use scraper::selector::ToCss;


#[derive(Debug, Deserialize)]
struct ScraperRequest {
    url: String,
    selector: String,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    headers: Vec<String>,
}


// Creating custom error that represents errors the scraper can encounter
#[derive(Debug)]
enum ScraperError {
    HttpRequestError(ReqwestError),
    ParseError(String),
}

// Implements the display for the custom errors
impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ScraperError::HttpRequestError(ref err) => write!(f, "Http Request Faied: {err}"),
            ScraperError::ParseError(ref desc) => write!(f, "Parsing Failed: {desc}")
        }
    }
}

// Need to implement the StdError trait for the custom error so it's compatible with rust's error library
impl StdError for ScraperError {
    // causing chain of errors if applicable
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            ScraperError::HttpRequestError(ref err) => Some(err),
            ScraperError::ParseError(_) => None,
        }
    }
}

// Convert Reqwest errors into our custom error type.
impl From<ReqwestError> for ScraperError {
    fn from(err: ReqwestError) -> ScraperError {
        ScraperError::HttpRequestError(err)
    }
}


#[post("/scrape", format = "json", data = "<scraper_request>")]
async fn scrape(scraper_request: Json<ScraperRequest>) -> Result<Json<Vec<String>>, String> {
    let headers = scrape_data_at_url(&scraper_request.url, &scraper_request.selector).await
        .map_err(|e| e.to_string())?;
    println!("{headers:?}");
    return Ok(Json(headers))
}

async fn scrape_data_at_url(url: &str, selector_str: &str) -> Result<Vec<String>, ScraperError> {
    let body = get(url).await?.text().await?;
    let error_string = format!("Failed to parse '{selector_str}' selector");
    let document = Html::parse_document(&body);
    let selector = Selector::parse(selector_str).map_err(|_| ScraperError::ParseError(error_string))?;

    println!("{}", selector.to_css_string());

    let headers = document.select(&selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect();
    Ok(headers)
}

#[launch]
fn rocket() -> Rocket<rocket::Build> {
    rocket::build().mount("/", routes![scrape])
}


// #[tokio::main]
// // async fn main() -> c_void {
// async fn main() -> Result<(), ScraperError> {
//     let mut url = "https://www.rust-lang.org";
//
//     let headers = match scrape_data_at_url(&url).await {
//         Ok(headers) => {
//             for header in &headers {
//                 println!("{:?}", header);
//             }
//             // Ok(())
//             headers
//         }
//         Err(e) => {
//             println!("An error occurred: {e}");
//             // Err(e)
//             return Err(e);
//         }
//     };
//
//
//     for header in headers {
//         println!("{:?}", header);
//     }
//
//     Ok(())
// }