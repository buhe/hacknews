// use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: u32,
    pub title: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub url: String,
}

pub async fn query_list() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json::<Vec<u32>>()
        .await?;
    Ok(resp)
}

pub async fn get_item(item_id: u32) -> Result<Story, Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://hacker-news.firebaseio.com/v0/item/".to_owned()
            + item_id.to_string().as_str()
            + ".json",
    )
    .await?
    .json::<Story>()
    .await?;
    Ok(resp)
}

pub async fn get_items(start: usize, len: usize) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let r = query_list().await?;
    let mut r2 = Vec::<u32>::new();
    let s = &r[start..start + len];
    r2.extend_from_slice(s);
    Ok(r2)
}

#[cfg(test)]
mod tests {
    use crate::sdk::{get_item, get_items};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_get_items() {
        let resp = tokio_test::block_on(get_items(0, 10));
        // println!("{:#?}", resp.unwrap());
        let len = resp.unwrap().len();
        assert_eq!(len, 10);
    }

    #[test]
    fn test_get_item() {
        let resp = tokio_test::block_on(get_item(100));
        assert_eq!(resp.unwrap().title.is_empty(), false);
    }
}
