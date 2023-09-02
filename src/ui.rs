use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::{Backend, Alignment},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io::Stdout, time::Duration};

use crate::app::App;
#[derive(PartialEq)]
pub enum UIPage {
    Providers,
    ProviderForm(String),
}

fn header() -> Paragraph<'static> {
    Paragraph::new("RUSTY TERRAFORM WIZARD").alignment(Alignment::Center).style(Style::default().fg(Color::LightMagenta))
}


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, current_page: &UIPage) {
    let p = header();
    f.render_widget(p, f.size());
    match current_page {
        UIPage::Providers => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = app
                .items
                .providers
                .iter()
                .map(|i| {
                    let lines = vec![Line::from(*i)];
                    ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Black))
                })
                .collect();

            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Providers"))
                .highlight_style(
                    Style::default()
                        .bg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            // We can now render the item list
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);
        }

        UIPage::ProviderForm(name) => {
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        //Constraint::Length(3),
                    ])
                .split(f.size());
            let block = Block::default().borders(Borders::ALL).title(name.to_string()).style(Style::default().bg(Color::Black).fg(Color::LightMagenta));
            f.render_widget(block, chunks[0]);
            //let text_box = Block::default().borders(Borders::ALL);
            //f.render_widget(text_box, chunks[1]);
        }
    }
}

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
    mut current_page: UIPage,
) -> Result<(), Box<dyn Error>> {
    app.items.begin(); // Highligts the first item in the list
    loop {
        match current_page {
            UIPage::Providers => {
                terminal.draw(|f| ui(f, &mut app, &current_page))?;
                if event::poll(Duration::from_millis(250))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Esc => return Ok(()),
                                KeyCode::Down => app.items.next(),
                                KeyCode::Up => app.items.previous(),
                                KeyCode::Enter => {
                                    let item = app.items.select();
                                    let name = item.provider_name;
                                    current_page = UIPage::ProviderForm(name)
                                }
                                _ => {}
                            }
                         }
                    }
                }
            }
            UIPage::ProviderForm(ref _name) => {
                terminal.draw(|f| ui(f, &mut app, &current_page))?;
                if event::poll(Duration::from_millis(250))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Esc => return Ok(()),
                                KeyCode::Left => current_page = UIPage::Providers,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
