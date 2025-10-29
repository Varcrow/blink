use crate::blink::{
    app::{App, Preview},
    file_style::{get_file_color_enhanced, get_file_icon_enhanced},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};

pub fn render_app(app: &App, frame: &mut Frame) {
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
    let dir_text = Paragraph::new(format!(" {}", app.cwd.display()))
        .style(Style::default().fg(app.config.colors.status_bar.to_ratatui_color()));
    frame.render_widget(Clear, area);
    frame.render_widget(dir_text, area);
}

fn render_parent_dir(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .parent_dir_entries
        .iter()
        .map(|entry| {
            let icon = get_file_icon_enhanced(entry);
            let style = Style::default().fg(get_file_color_enhanced(entry));
            let entry_str = format!("{} {}", icon, entry.name);

            let line = Line::from(vec![Span::styled(entry_str, style)]);

            ListItem::new(line)
        })
        .collect();

    let parent_list = List::new(items).block(
        Block::bordered()
            .border_type(app.config.ui.get_border_type())
            .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
    );

    frame.render_widget(Clear, area);
    frame.render_widget(parent_list, area);
}

fn render_current_dir(app: &App, frame: &mut Frame, area: Rect) {
    let width = area.width as usize;

    let items: Vec<ListItem> = app
        .cwd_entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let icon = get_file_icon_enhanced(entry);
            let mut style = Style::default().fg(get_file_color_enhanced(entry));

            if app.visual_mode && app.visual_selection.contains(&idx) {
                style = style
                    .bg(app.config.colors.selected_bg.to_ratatui_color())
                    .add_modifier(Modifier::BOLD);
            }

            let size_str = format!("{}", entry.size);
            let entry_str = format!("{} {}", icon, entry.name);
            let pad_len = width.saturating_sub(entry_str.len() + size_str.len());
            let padding = " ".repeat(pad_len);

            let line = Line::from(vec![
                Span::styled(entry_str, style),
                Span::raw(padding),
                Span::raw(size_str),
            ]);

            ListItem::new(line)
        })
        .collect();

    let cwd_list = List::new(items)
        .scroll_padding((area.height / 2) as usize) // GHETO SCROLL FIX
        .block(
            Block::bordered()
                .border_type(app.config.ui.get_border_type())
                .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
        )
        .highlight_style(
            Style::default()
                .bg(app.config.colors.selected_bg.to_ratatui_color())
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(cwd_list, area, &mut app.list_state.clone());
}

fn render_preview_dir(app: &App, frame: &mut Frame, area: Rect) {
    let preview = if let Some(idx) = app.list_state.selected() {
        if let Some(selected_entry) = app.cwd_entries.get(idx) {
            if let Ok(guard) = selected_entry.preview.lock() {
                guard.clone()
            } else {
                Preview::Binary {
                    info: "Loading...".to_string(),
                }
            }
        } else {
            // No entry at selected index
            Preview::Binary {
                info: "".to_string(),
            }
        }
    } else {
        // No selection or empty directory
        Preview::Binary {
            info: "".to_string(),
        }
    };

    match preview {
        Preview::File { contents } => {
            let file_contents = Paragraph::new(contents).block(
                Block::bordered()
                    .border_type(app.config.ui.get_border_type())
                    .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
            );
            frame.render_widget(Clear, area);
            frame.render_widget(file_contents, area);
        }
        Preview::Directory { entries } => {
            let preview_contents: Vec<ListItem> = entries
                .iter()
                .map(|entry| {
                    let icon = get_file_icon_enhanced(entry);
                    let style = Style::default().fg(get_file_color_enhanced(entry));
                    let entry_str = format!("{} {}", icon, entry.name);
                    let line = Line::from(vec![Span::styled(entry_str, style)]);
                    ListItem::new(line)
                })
                .collect();
            let preview_list = List::new(preview_contents).block(
                Block::bordered()
                    .border_type(app.config.ui.get_border_type())
                    .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
            );
            frame.render_widget(Clear, area);
            frame.render_widget(preview_list, area);
        }
        Preview::Image { path } => {
            let content = Paragraph::new(path.display().to_string()).block(
                Block::bordered()
                    .border_type(app.config.ui.get_border_type())
                    .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
            );
            frame.render_widget(Clear, area);
            frame.render_widget(content, area);
        }
        Preview::Binary { info } => {
            let content = Paragraph::new(info).block(
                Block::bordered()
                    .border_type(app.config.ui.get_border_type())
                    .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
            );
            frame.render_widget(Clear, area);
            frame.render_widget(content, area);
        }
    }
}

fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let dir_count = app.cwd_entries.iter().filter(|e| e.is_dir).count();
    let file_count = app.cwd_entries.len() - dir_count;
    let yank_status = if app.yanked_entry_paths.is_some() && app.is_cut == true {
        "[Cut]"
    } else if app.yanked_entry_paths.is_some() {
        "[Yanked]"
    } else {
        "[Clear]"
    };

    let status = format!(
        " 📁 {} dirs | 📄 {} files | {} ",
        dir_count, file_count, yank_status
    );

    let status_bar = Paragraph::new(status)
        .style(Style::default().fg(app.config.colors.status_bar.to_ratatui_color()));

    frame.render_widget(Clear, area);
    frame.render_widget(status_bar, area);
}

// Universal function for states that need to render input
pub fn render_input_popup(app: &App, frame: &mut Frame, title: String, content: String) {
    let area = centered_rect(30, 15, frame.area());
    let popup = Paragraph::new(content)
        .block(
            Block::bordered()
                .title(title)
                .title_alignment(Alignment::Center)
                .border_type(app.config.ui.get_border_type())
                .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
        )
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

pub fn render_input_prompt_popup(app: &App, frame: &mut Frame, title: String, content: String) {
    let area = centered_rect(30, 15, frame.area());
    let popup = Paragraph::new(content)
        .block(
            Block::bordered()
                .title(title)
                .title_alignment(Alignment::Center)
                .border_type(app.config.ui.get_border_type())
                .border_style(Style::default().fg(app.config.colors.border.to_ratatui_color())),
        )
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

pub fn render_bookmark_list(app: &App, frame: &mut Frame, list_state: &mut ListState) {
    let area = centered_rect(60, 60, frame.area());

    let items: Vec<ListItem> = app
        .bookmarks
        .list()
        .iter()
        .map(|(name, entry)| {
            let line = Line::from(vec![Span::raw(format!(
                "{} {}: {}",
                "\u{eaa5}".to_string(),
                name,
                entry.path.to_string_lossy()
            ))]);
            ListItem::new(line)
        })
        .collect();

    let bookmark_list = List::new(items)
        .block(
            Block::bordered()
                .title("Bookmarks")
                .title_alignment(Alignment::Center)
                .border_type(app.config.ui.get_border_type())
                .style(Style::default().fg(app.config.colors.status_bar.to_ratatui_color())),
        )
        .highlight_style(
            Style::default()
                .bg(app.config.colors.selected_bg.to_ratatui_color())
                .add_modifier(Modifier::BOLD),
        );

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
