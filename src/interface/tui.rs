use crate::sdk::Story;
use cursive::views::{TextView,ListView};

pub fn show(items: Vec<Story>) {
     let mut siv = cursive::default();
    let mut l = ListView::new();
    for i in 0..items.len() - 1 {
        let s = &items[i];
        l.add_child(&s.id.to_string(),TextView::new(&s.title));
    }
    // Creates a dialog with a single "Quit" button
    siv.add_layer(l);
    // Starts the event loop.
    siv.run();
} 