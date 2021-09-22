use crate::{next, prev, sdk::Story};
use cursive::{Cursive, CursiveRunnable, traits::Boxable, view::SizeConstraint, views::{Button, DummyView, LinearLayout, ListView, ResizedView, TextView}};
pub struct UI {
    siv: CursiveRunnable,
}
impl UI {
    pub fn new() -> UI {
        let siv = cursive::default();

        UI { siv: siv }
    }
    pub fn show(&mut self, items: Vec<Story>) {
        let mut l = ListView::new();
        for i in 0..items.len() - 1 {
            let s = &items[i];
            l.add_child(&s.id.to_string(), TextView::new(&s.title));
        }
        let buttons = LinearLayout::horizontal()
            .child(Button::new("Prev", prev))
            .child(Button::new("Next", next))
            .child(DummyView.fixed_width(15))
            .child(Button::new("Quit", Cursive::quit));
        l.add_child(
            "Select",
            ResizedView::new(
                SizeConstraint::Fixed(50),
                SizeConstraint::Fixed(1),
                buttons,
            ),
        );
        // let c = &self.siv;
        l.set_on_select(|c, s| c.add_global_callback('w', |s| s.quit()));
        self.siv.add_layer(l);
        self.siv.add_global_callback('q', |s| s.quit());
        self.siv.run();
    }
    pub fn flush(&mut self) {
        self.siv.clear();
    }

    pub fn select(&mut self) {
        self.flush();
    }
}
