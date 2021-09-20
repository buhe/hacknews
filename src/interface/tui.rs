use crate::{next, prev, sdk::Story};
use cursive::{Cursive, view::SizeConstraint, views::{Button, DummyView, LinearLayout, ListView, ResizedView, TextView}};

pub fn show(items: Vec<Story>) {
     let mut siv = cursive::default();
    let mut l = ListView::new();
    for i in 0..items.len() - 1 {
        let s = &items[i];
        l.add_child(&s.id.to_string(),TextView::new(&s.title));
    }
    // Creates a dialog with a single "Quit" button
   
    //   let select = SelectView::<String>::new()
    //     // .on_submit(on_submit)
    //     // .with_name("select")
    //     .fixed_size((10, 5));
    let buttons = LinearLayout::horizontal()
        .child(Button::new("Prev", prev))
        .child(Button::new("Next", next))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));
        let buttons2 = LinearLayout::vertical().child(buttons);
    l.add_child("Select",ResizedView::new(SizeConstraint::Fixed(20),
                                SizeConstraint::Fixed(1),
                                buttons2));
    siv.add_layer(l);
    // siv.add_layer(Dialog::around(LinearLayout::vertical()
    //         // .child(select)
    //         .child(DummyView)
    //         .child(DummyView)
    //         .child(DummyView)
    //         .child(buttons))
    //     .title("Select a profile"));

    // Starts the event loop.
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
} 