use crate::providers::Providers;

pub struct App<'a> {
    pub items: Providers<&'a str>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: Providers::with_items(vec!["AWS", "GOOGLE", "AZURE", "DOCKER"]),
        }
    }
}
