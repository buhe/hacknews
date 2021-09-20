// use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    id: Option<i32>,
    title: String,
}

pub async fn query_list(_max_id: u32) -> Result<Vec<u32>, Box<dyn std::error::Error>>  {
    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json::<Vec<u32>>()
        .await?;
    Ok(resp)
}

pub async fn get_item(item_id: u32) -> Result<Story, Box<dyn std::error::Error>>  {
    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/item/".to_owned() + item_id.to_string().as_str() +".json")
        .await?
        .json::<Story>()
        .await?;
    Ok(resp)
}
