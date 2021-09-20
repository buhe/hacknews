

use std::{error, sync::Mutex};

use cursive::Cursive;
use sdk::Story;

use crate::sdk::{get_item, get_items};
mod sdk;
mod interface;
const PAGE_SIZE: usize = 10;
 #[macro_use] 
 extern crate lazy_static;
 lazy_static! {
   static ref INDEX: Mutex<usize> = Mutex::new(0);
 }
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let resp = get_items(*INDEX.lock().unwrap(),PAGE_SIZE).await?;
    let mut ss = Vec::<Story>::new();
      for i in 0..resp.len() - 1 {
        ss.push(get_item(resp[i]).await?);
      }
    let mut ui = interface::tui::UI::new();
    ui.show(ss);
    Ok(())
}

pub fn next(_c: &mut Cursive) {
    *INDEX.lock().unwrap() += 10;
}
pub fn prev(_c: &mut Cursive) {
    if *INDEX.lock().unwrap() > 0 {
        *INDEX.lock().unwrap() -= 10;
    }
}
