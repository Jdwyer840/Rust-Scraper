use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct WebScrape {
    data: Data,
    steps: Steps,
}

#[derive(Debug, Deserialize)]
struct Data {
    url: String,
}

#[derive(Debug, Deserialize)]
struct Steps {
    step: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct Step {
    action: String,
    url: String,
}