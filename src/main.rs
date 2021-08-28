extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use reqwest::Error;

#[derive(Debug, Deserialize, PartialEq)]
struct Author {
    name: String
}

#[derive(Debug, Deserialize, PartialEq)]
struct Entry {
    #[serde(rename = "author", default)]
    authors: Vec<Author>,
    id: String,
    published: String,
    title: String,
    summary: String,
}


#[derive(Debug, Deserialize, PartialEq)]
struct Feed {
    #[serde(rename = "entry", default)]
    entries: Vec<Entry>,
    title: String
}


async fn arxiv_request(url: &String) -> Result<String, Error>{
    println!("{}", url);
    let response = reqwest::get(url).await?;
    
    let body = response.text().await?;

    return Ok(body)
}

fn parse_xml(body: &str) -> Result<Feed, DeError> {
    let feed: Feed = from_str(body)?;
    return Ok(feed);
}

#[tokio::main]
async fn main() {
    let request_url = String::from(
        "http://export.arxiv.org/api/query?search_query=all:\"real-time bidding\"+OR+all:\"online advertisment\"+cat:cs&sortBy=lastUpdatedDate&sortOrder=descending&max_results=4"
    );

    let xlm_body = arxiv_request(&request_url).await.expect(
        "error in the request of today"
    );

    let feed = parse_xml(&xlm_body[..]).expect(
        "Error in parsing the xlm body"
    );

    

}
