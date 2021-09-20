pub async fn queryList(maxId: u32) -> Result<Vec<u32>, Box<dyn std::error::Error>>  {
    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json::<Vec<u32>>()
        .await?;
    Ok(resp)
}