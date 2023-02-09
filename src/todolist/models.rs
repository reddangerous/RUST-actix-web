use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub date: String,
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
}