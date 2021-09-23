use crate::sdk::Story;

pub struct UI {}
impl UI {
    pub fn new() -> UI {
        UI {}
    }
    pub fn show(&mut self, items: Vec<Result<Story, reqwest::Error>>) {
        for i in 0..items.len() - 1 {
            let s = &items[i].as_ref().unwrap();
            println!("{}: {}", s.id, s.title);
        }
    }
    pub fn item(&self, item: &Story) {
        println!("{}", item.title);
        println!("{}", item.text);
        println!("{}", item.url);
    }
}
