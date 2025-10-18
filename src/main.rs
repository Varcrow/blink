use crate::stfm::model::{DirPreview, Model, RunningState};
use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{env::current_dir, time::Duration};

mod stfm;

fn main() -> color_eyre::Result<()> {
    // Init
    let mut terminal = ratatui::init();
    let mut model = Model::new(current_dir()?);

    // Loop
    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&mut model, frame))?;

        if event::poll(Duration::from_millis(100))? {
            handle_input(&mut model)?;
        }
    }

    // Restore
    ratatui::restore();
    Ok(())
}

// Handles way keys are pressed
fn handle_input(model: &mut Model) -> color_eyre::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => model.running_state = RunningState::Done,
                KeyCode::Down | KeyCode::Char('j') => model.next(),
                KeyCode::Up | KeyCode::Char('k') => model.previous(),
                KeyCode::Left | KeyCode::Char('h') => model.up_dir_level(),
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    model.enter_selected();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

// Render func
fn view(model: &mut Model, frame: &mut Frame) {
    // Three sections of layout: Parent | Current | Preview
    let layout = Layout::horizontal([
        Constraint::Fill(2),
        Constraint::Fill(2),
        Constraint::Fill(6),
    ])
    .split(frame.area());

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
        .highlight_symbol("> ");

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

    let parent_list =
        List::new(items).block(Block::default().borders(Borders::ALL).title(format!(
            "{}",
            model
                .current_dir
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "Root".to_string())
        )));

    // Preview contents
    match &model.dir_preview {
        DirPreview::File { contents } => {
            let file_contents =
                Paragraph::new(contents.clone()).block(Block::default().borders(Borders::ALL));
            frame.render_widget(file_contents, layout[2]);
        }
        DirPreview::Directory { entries } => {
            let preview_contents: Vec<ListItem> = entries
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

            let preview_list = List::new(preview_contents).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", model.current_dir.display())),
            );
            frame.render_widget(preview_list, layout[2]);
        }
    }

    // Render lists
    frame.render_stateful_widget(cwd_list, layout[1], &mut model.list_state);
    frame.render_widget(parent_list, layout[0]);
}
