use crate::ui::{UIPage, self};

#[derive(Debug)]
pub struct ProviderForm {
    pub provider_name: String
}

impl ProviderForm {
    pub fn new_provider_form(name: String) -> UIPage {
        ui::UIPage::ProviderForm(name) 
    }
}