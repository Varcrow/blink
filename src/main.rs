use crate::stfm::model::{Model, RunningState};
use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};
use std::env::current_dir;

mod stfm;

fn main() -> color_eyre::Result<()> {
    // Init
    let mut terminal = ratatui::init();
    let mut model = Model::new(current_dir()?);

    // Loop
    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&mut model, frame))?;
        handle_input(&mut model);
    }

    // Restore
    ratatui::restore();
    Ok(())
}

// Handles way keys are pressed
fn handle_input(model: &mut Model) {
    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => model.running_state = RunningState::Done,
                KeyCode::Down | KeyCode::Char('j') => model.next(),
                KeyCode::Up | KeyCode::Char('k') => model.previous(),
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    model.enter_selected();
                }
                _ => {}
            }
        }
    }
}

// Render func
fn view(model: &mut Model, frame: &mut Frame) {
    // Three sections of layout: Parent | Current | Preview
    let layout = Layout::horizontal([
        Constraint::Fill(3),
        Constraint::Fill(4),
        Constraint::Fill(3),
    ])
    .split(frame.area());

    // Three borders, one for each section
    frame.render_widget(Block::bordered().border_set(border::ROUNDED), layout[2]);

    //Make the cwd list
    let items: Vec<ListItem> = model
        .cwd_entries
        .iter()
        .map(|entry| {
            let icon = if entry.is_dir { "📁" } else { "📄" };
            let style = if entry.is_dir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::raw(format!("{} ", icon)),
                Span::styled(&entry.name, style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let cwd_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", model.current_dir.display())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    //Make the parent dir list
    let items: Vec<ListItem> = model
        .parent_dir_entries
        .iter()
        .map(|entry| {
            let icon = if entry.is_dir { "📁" } else { "📄" };
            let style = if entry.is_dir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::raw(format!("{} ", icon)),
                Span::styled(&entry.name, style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let parent_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("{}", model.current_dir.parent().unwrap().display())),
    );

    // Render lists
    frame.render_widget(parent_list, layout[0]);
    frame.render_stateful_widget(cwd_list, layout[1], &mut model.list_state);
}
