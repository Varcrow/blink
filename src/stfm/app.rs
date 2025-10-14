use crate::stfm::operations::get_all_entries_in_cwd;
use color_eyre::eyre::Ok;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Alignment;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    symbols::border,
    widgets::Block,
};

pub struct App {
    input: String,
    running: bool,
    cwd_entries: Vec<String>,
}

#[derive(PartialEq)]
enum Message {
    Make { t: String, path: String },
    Remove { path: String },
    Quit,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            running: true,
            cwd_entries: get_all_entries_in_cwd().unwrap(),
        }
    }
}

impl App {
    pub fn run(mut self) -> color_eyre::Result<()> {
        let mut terminal = ratatui::init();
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => self.running = false,
                    _ => {}
                }
            }
        }
        ratatui::restore();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout =
            Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(3)]).split(frame.area());
        // Input line
        frame.render_widget(
            Paragraph::new("[stfm]> ").block(Block::bordered().border_set(border::ROUNDED)),
            layout[1],
        );

        // List logic
        let items = self
            .cwd_entries
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .border_set(border::ROUNDED)
                    .title("Simple Text File Manager")
                    .title_alignment(Alignment::Center)
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        frame.render_widget(list, layout[0]);
    }
}
