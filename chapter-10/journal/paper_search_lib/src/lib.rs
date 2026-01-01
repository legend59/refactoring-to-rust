use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub entry: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub updated: String,
    pub published: String,
    pub title: String,
    pub summary: String,
    pub author: Vec<Author>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
}

pub async fn search(term: String, page: isize, max_results: isize) -> Result<Feed, reqwest::Error> {
    let http_response = reqwest::get(format!(
        // "http://export.arxiv.org/api/query?search_query=all:{}&start={}&max_results={}",
        // A node proxy server is required to run this.
        // Please run it below.
        // cd ./chapter-10/arxiv-local-proxy
        // npm start
        "http://127.0.0.1:3000/api/query?search_query=all:{}&start={}&max_results={}",
        term,
        page * max_results,
        max_results
    ))
    .await?;
    let b = http_response.text().await?;
    let feed: Feed = serde_xml_rs::from_str(b.as_str()).unwrap();
    return Ok(feed);
}