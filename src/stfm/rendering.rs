use crate::stfm::app::{App, DirPreview};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Three sections of layout: Parent | Current | Preview
    let layout = Layout::horizontal([
        Constraint::Fill(2),
        Constraint::Fill(2),
        Constraint::Fill(6),
    ])
    .split(frame.area());

    //Make the cwd list
    let items: Vec<ListItem> = app
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
                .title(format!(" {} ", app.current_dir.display())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    //Make the parent dir list
    let items: Vec<ListItem> = app
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
            app
                .current_dir
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "Root".to_string())
        )));

    // Preview contents widget
    match &app.dir_preview {
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
                    .title(format!(" {} ", app.current_dir.display())),
            );
            frame.render_widget(preview_list, layout[2]);
        }
    }

    // Render lists
    frame.render_stateful_widget(cwd_list, layout[1], &mut app.list_state);
    frame.render_widget(parent_list, layout[0]);
}
