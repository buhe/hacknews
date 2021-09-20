use cursive::views::{Dialog, TextView,ListView};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json::<Vec<u32>>()
        .await?;
    println!("{:#?}", resp);
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
