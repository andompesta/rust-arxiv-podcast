use chrono::{DateTime, Utc};
use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use reqwest::Error;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
struct Author {
    name: String
}

#[derive(Debug, Deserialize, PartialEq)]
struct Entry {
    #[serde(rename = "author")]
    authors: Vec<Author>,
    id: String,
    published: DateTime<Utc>,
    title: String,
    summary: String,
    score: Option<f32>
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct Feed {
    #[serde(rename = "entry", default)]
    entries: Vec<Entry>,
    title: String
}

impl fmt::Display for Feed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "title: {} num_entries: {}", self.title, self.entries.len())
    }
}

pub async fn get_body() -> Result<String, Error>{
    let url = "http://export.arxiv.org/api/query?search_query=all:\"real-time bidding\"+OR+all:\"online advertisment\"+cat:cs&sortBy=lastUpdatedDate&sortOrder=descending&max_results=4";

    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    return Ok(body)
}

pub fn parse(body: &str) -> Result<Feed, DeError> {
    let feed: Feed = from_str(body)?;
    return Ok(feed);
}