use crate::providerform::*;
use ratatui::widgets::ListState;
pub struct Providers<T> {
    pub providers: Vec<T>,
    pub state: ListState,
}

impl<T> Providers<T> {
    pub fn new(providers: Vec<T>) -> Providers<T> {
        Providers {
            providers,
            state: ListState::default(),
        }
    }

    pub fn with_items(providers: Vec<T>) -> Providers<T> {
        Providers {
            state: ListState::default(),
            providers,
        }
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.providers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.providers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn begin(&mut self) {
        self.state.select(Some(0));
    }

    pub fn select(&mut self) -> ProviderForm {
        let i = self.state.selected().unwrap();
        let provider: &str = match i {
            0 => "AWS",
            1 => "GOOGLE",
            2 => "AZURE",
            3 => "DOCKER",
            _ => "None",
        };

        let fields: Vec<&str> = match provider {
            "AWS" => {
                vec!["Source", "Version", "Required Version", "Region", "Resource Type", "Resource Name", "AMI", "Instance Type", "Tags Name"]
            },
            _ => vec![]
        };

        // let provider_form = ProviderForm {
        //     provider_name: provider.to_string(),
        //     current_index: None,
        //     fields,
        //     state: ListState::default(),
            
        // };
        // ProviderForm {
        //     provider_name: provider_form.provider_name,
        //     current_index: provider_form.current_index,
        //     fields: provider_form.fields,
        //     state: provider_form.state,
        // }
        ProviderForm {
            provider_name: provider.to_string(),
            current_index: None,
            fields,
            state: ListState::default(),
        }
    }
}
