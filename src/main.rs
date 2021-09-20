

use sdk::Story;

use crate::sdk::{get_item, get_items};
mod sdk;
mod interface;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = get_items(0,10).await?;
    let mut ss = Vec::<Story>::new();
      for i in 0..resp.len() - 1 {
        ss.push(get_item(resp[i]).await?);
      }
    interface::tui::show(ss);
    Ok(())
}
