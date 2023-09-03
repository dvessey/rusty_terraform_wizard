use ratatui::{style::{Color, Style}, widgets::ListState};

use crate::ui::{self, UIPage};

#[derive(Debug)]
pub struct ProviderForm<'a> {
    pub provider_name: String,
    pub current_index: Option<usize>,
    pub fields: Vec<&'a str>,
    pub state: ListState,
}

impl<'a> ProviderForm<'a> {
    // pub fn new(name: String) -> ProviderForm<'a> {
    //     ProviderForm { provider_name: name, current_index: Some(0), fields: vec![], state: ListState::default() }
    // }

    pub fn new_provider_form(name: String) -> UIPage {
        ui::UIPage::ProviderForm(name)
    }

    pub fn border_style(&self, idx: usize) -> Style {
        if self.current_index == Some(idx) {
            Style::default().fg(Color::White)
        } else {
            Style::default()
        }
    }
}
