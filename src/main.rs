use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

use terraform_wizard_rs::{app::App, ui::*};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::new();

    let current_page = UIPage::Providers;
    let focused: bool = false;
    // let provider: ProviderForm = ProviderForm { provider_name: "AWS".to_string(), current_index: Some(0), fields:  vec!["Source", "Version", "Required Version", "Region", "Resource Type", "Resource Name", "AMI", "Instance Type", "Tags Name"], state: ListState::default() };

    run(&mut terminal, app, current_page, focused)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
