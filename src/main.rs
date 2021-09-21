use std::{error, sync::Mutex};

use crate::sdk::{get_item, get_items};
use clap::{AppSettings, Clap};
use cursive::Cursive;
use sdk::Story;
mod interface;
mod sdk;
const PAGE_SIZE: usize = 10;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref INDEX: Mutex<usize> = Mutex::new(0);
}
#[derive(Clap)]
#[clap(version = "1.0", author = "buhe")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = "cli")]
    ui: String,
    #[clap(short, long, required = false, default_value = "")]
    item: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opts: Opts = Opts::parse();
    let resp = get_items(*INDEX.lock().unwrap(), PAGE_SIZE).await?;
    let mut ss = Vec::<Story>::new();
    for i in 0..resp.len() - 1 {
        ss.push(get_item(resp[i]).await?);
    }
    if opts.ui == "cli" {
        let mut ui = interface::cli::UI::new();
        if opts.item.is_empty() {
            ui.show(ss);
        } else {
            let id = opts.item;
            let item = get_item(id.parse().unwrap()).await?;
            ui.item(&item);
        }
    } else {
        let mut ui = interface::tui::UI::new();
        ui.show(ss);
    }
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
