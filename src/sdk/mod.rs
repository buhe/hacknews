// use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    id: Option<i32>,
    title: String,
}

pub async fn query_list() -> Result<Vec<u32>, Box<dyn std::error::Error>>  {
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

pub async fn get_items(start: u32, len: u32) -> Result<Vec<u32>, Box<dyn std::error::Error>>  {
    let r = query_list().await?;
    let mut r2 = Vec::<u32>::new();
    r2.append(&mut r[0..1].to_owned());
    Ok(r2)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // async fn test_query_list() -> Result<(), Box<dyn std::error::Error>> {
    //     let resp = get_items(0,10).await?;
    //     println!("{:#?}", resp);
    //     Ok(())
    // }
}
