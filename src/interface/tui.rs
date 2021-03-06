use crate::{next, prev, sdk::Story};
use cursive::{
    traits::Boxable,
    view::SizeConstraint,
    views::{Button, DummyView, LinearLayout, ListView, ResizedView, TextView},
    Cursive, CursiveRunnable,
};
pub struct UI {
    siv: CursiveRunnable,
}
impl UI {
    pub fn new() -> UI {
        let siv = cursive::default();

        UI { siv: siv }
    }
    pub fn show(&mut self, items: Vec<Result<Story, reqwest::Error>>) {
        let mut l = ListView::new();
        for i in 0..items.len() - 1 {
            let s = items[i].as_ref().unwrap();
            let title = s.title.clone();
            let text = s.text.clone();
            let url = s.url.clone();
            l.add_child(
                &s.id.to_string(),
                LinearLayout::horizontal().child(Button::new(&s.title, move |c| {
                    c.pop_layer();
                    c.add_layer(
                        LinearLayout::vertical()
                            .child(TextView::new(&title))
                            .child(DummyView)
                            .child(TextView::new(&text))
                            .child(DummyView)
                            .child(TextView::new(&url))
                            .child(Button::new("Back", |c|{
                                c.pop_layer();
                            }))
                            ,
                    );
                })),
            );
        }
        let buttons = LinearLayout::horizontal()
            .child(Button::new("Prev", prev))
            .child(Button::new("Next", next))
            .child(DummyView.fixed_width(15))
            .child(Button::new("Quit", Cursive::quit));
        l.add_child(
            "Select",
            ResizedView::new(SizeConstraint::Fixed(50), SizeConstraint::Fixed(1), buttons),
        );
        self.siv.add_layer(l);
        self.siv.add_global_callback('q', |s| s.quit());
        self.siv.run();
    }
}
