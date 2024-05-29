
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs;
use anyhow::Error;
pub use types::*;
use crate::xml_parsing::types::WebScrape;
pub mod types;

fn parse_xml(xml: &str) -> Result<WebScrape, Error> {
    let parsed_xml: WebScrape = from_str(xml)?;
    Ok(parsed_xml)
}