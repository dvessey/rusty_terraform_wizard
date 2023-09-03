use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Backend},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io::Stdout, time::Duration};

use crate::app::App;
//#[derive(PartialEq)]
pub enum UIPage {
    Providers,
    ProviderForm(String),
}

fn header() -> Paragraph<'static> {
    Paragraph::new("RUSTY TERRAFORM WIZARD - by Damon Vessey")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::LightMagenta))
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, current_page: &UIPage, focused: bool) {
    let p = header();
    //let index = 0; //used to keep track of block to highlight and insert text
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
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(f.size());
            let paragraph = Paragraph::new(name.to_string())
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks[0]);

            let border_style = match focused {
                true => Style::default().fg(Color::White),
                false => Style::default().fg(Color::LightMagenta),
            };

            let border_type = match focused {
                true => BorderType::Thick,
                false => BorderType::Plain,
            };

            match name.as_str() {
                "AWS" => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Source")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(block, chunks[1]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Version")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[2]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Required Version")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[3]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Region")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[4]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Type")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[5]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[6]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("AMI")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[7]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Instance Type")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[8]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Tags Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[9]);
                }
                "AZURE" => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Source")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(block, chunks[1]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Version")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[2]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Required Version")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[3]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Type")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[4]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[5]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[6]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Location")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[7]);
                }
                "GOOGLE" => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Source")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(block, chunks[1]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Version")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[2]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Credentials")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[3]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Project ID")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[4]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Region")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[5]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Zone")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[6]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Type")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[7]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[8]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[9]);
                }
                "DOCKER" => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Source")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(block, chunks[1]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Version")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[2]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Type")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[3]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[4]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[5]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Keep Locally")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[6]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Type")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[7]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Resource Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[8]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Image")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[9]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Name")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[10]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("Internal Port")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[11]);
                    let text_box = Block::default()
                        .borders(Borders::ALL)
                        .title("External Port")
                        .style(border_style)
                        .border_type(border_type);
                    f.render_widget(text_box, chunks[12]);
                }

                _ => {}
            };
        }
    }
}

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
    mut current_page: UIPage,
    mut focused: bool,
) -> Result<(), Box<dyn Error>> {
    app.items.begin(); // Highligts the first item in the list
    loop {
        match current_page {
            UIPage::Providers => {
                terminal.draw(|f| ui(f, &mut app, &current_page, false))?;
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
                terminal.draw(|f| ui(f, &mut app, &current_page, focused))?;
                if event::poll(Duration::from_millis(250))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Esc => return Ok(()),
                                KeyCode::Left => current_page = UIPage::Providers,
                                KeyCode::Down => {
                                    focused = true;
                                }
                                KeyCode::Up => {
                                    focused = false;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
