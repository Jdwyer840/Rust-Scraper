use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    actions: Actions
}

#[derive(Debug, Deserialize)]
struct Actions {
    go_to: String,
}