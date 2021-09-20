use cursive::views::{Dialog, TextView,ListView};

use crate::sdk::{get_item, get_items, query_list};
mod sdk;
mod interface;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = get_items(0,10).await?;
    // println!("{:#?}", resp);
    // let resp2 = get_item(resp.first().unwrap().to_owned()).await?;
    // println!("{:#?}", resp2);
    // Creates the cursive root - required for every application.
    // let mut siv = cursive::default();
    // let mut l = ListView::new();
    // l.add_child("1",TextView::new("2"));
    // // Creates a dialog with a single "Quit" button
    // siv.add_layer(l);

    // // Starts the event loop.
    // siv.run();
    Ok(())
}
