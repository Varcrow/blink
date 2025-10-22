use crate::stfm::{
    app::{App, Preview},
    file_style::{get_file_color, get_file_color_enhanced, get_file_icon, get_file_icon_enhanced},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

pub fn render_main_state(app: &App, frame: &mut Frame) {
    let outer_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let columns = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(50),
    ])
    .split(outer_layout[1]);

    render_current_dir_text(app, frame, outer_layout[0]);
    render_parent_dir(app, frame, columns[0]);
    render_current_dir(app, frame, columns[1]);
    render_preview_dir(app, frame, columns[2]);
    render_status_bar(app, frame, outer_layout[2]);
}

fn render_current_dir_text(app: &App, frame: &mut Frame, area: Rect) {
    let dir_text =
        Paragraph::new(format!(" {}", app.cwd.display())).style(Style::default().fg(Color::White));
    frame.render_widget(dir_text, area);
}

fn render_parent_dir(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .parent_dir_entries
        .iter()
        .map(|entry| {
            let icon = get_file_icon_enhanced(entry);
            let style = Style::default().fg(get_file_color_enhanced(entry));

            let line = Line::from(vec![
                Span::raw(format!("{} ", icon)),
                Span::styled(&entry.name, style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let parent_list = List::new(items).block(Block::default().borders(Borders::ALL));

    frame.render_widget(parent_list, area);
}

fn render_current_dir(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .cwd_entries
        .iter()
        .map(|entry| {
            let icon = get_file_icon_enhanced(entry);
            let style = Style::default().fg(get_file_color_enhanced(entry));

            let line = Line::from(vec![
                Span::raw(format!("{} ", icon)),
                Span::styled(&entry.name, style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let cwd_list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    frame.render_stateful_widget(cwd_list, area, &mut app.list_state.clone());
}

fn render_preview_dir(app: &App, frame: &mut Frame, area: Rect) {
    match &app.preview_contents {
        Preview::File { contents } => {
            let file_contents =
                Paragraph::new(contents.clone()).block(Block::default().borders(Borders::ALL));
            frame.render_widget(file_contents, area);
        }
        Preview::Directory { entries } => {
            let preview_contents: Vec<ListItem> = entries
                .iter()
                .map(|entry| {
                    let icon = get_file_icon_enhanced(entry);
                    let style = Style::default().fg(get_file_color_enhanced(entry));

                    let line = Line::from(vec![
                        Span::raw(format!("{} ", icon)),
                        Span::styled(&entry.name, style),
                    ]);

                    ListItem::new(line)
                })
                .collect();

            let preview_list =
                List::new(preview_contents).block(Block::default().borders(Borders::ALL));
            frame.render_widget(preview_list, area);
        }
    }
}

fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let dir_count = app.cwd_entries.iter().filter(|e| e.is_dir).count();
    let file_count = app.cwd_entries.len() - dir_count;
    let yank_status = if app.yanked_entry_path.is_some() && app.is_cut == true {
        "[Cut]"
    } else if app.yanked_entry_path.is_some() {
        "[Yanked]"
    } else {
        "[Clear]"
    };

    let status = format!(
        " 📁 {} dirs | 📄 {} files | {} ",
        dir_count, file_count, yank_status
    );

    let status_bar = Paragraph::new(status).style(Style::default().fg(Color::White));

    frame.render_widget(status_bar, area);
}

// Universal function for states that need to render input
pub fn render_input_popup(frame: &mut Frame, title: String, content: String) {
    let area = centered_rect(30, 15, frame.area());
    let popup = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Center)
                .style(Style::default()),
        )
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

pub fn render_confirm_delete_popup(frame: &mut Frame) {
    let area = centered_rect(30, 15, frame.area());
    let popup = Paragraph::new("y / n")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Delete")
                .title_alignment(Alignment::Center)
                .style(Style::default()),
        )
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

pub fn render_bookmark_list(app: &App, frame: &mut Frame, list_state: &mut ListState) {
    let area = centered_rect(30, 30, frame.area());

    let items: Vec<ListItem> = app
        .bookmarks
        .list()
        .iter()
        .map(|(name, entry)| {
            let line = Line::from(vec![Span::raw(format!(
                "{}{}: {}",
                "🔖".to_string(),
                name,
                entry.path.to_string_lossy()
            ))]);
            ListItem::new(line)
        })
        .collect();

    let bookmark_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Bookmarks")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(bookmark_list, area, list_state);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
