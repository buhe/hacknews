use crate::sdk::Story;

pub struct UI {}
impl UI {
    pub fn new() -> UI {
        UI {}
    }
    pub fn show(&mut self, items: Vec<Story>) {
        for i in 0..items.len() - 1 {
            let s = &items[i];
            println!("{}: {}", s.id, s.title);
        }
    }
    pub fn item(&self, item: &Story) {
        println!("{}", item.title);
        println!("{}", item.text);
        println!("{}", item.url);
    }
}
