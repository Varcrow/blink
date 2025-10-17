use std::env::current_dir;

use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    symbols::border,
    widgets::Block,
};

use crate::stfm::message::Message;
use crate::stfm::model::{Model, RunningState};
use crate::stfm::entries::get_entries;

mod stfm;

fn main() -> color_eyre::Result<()> {
    // Init
    let mut terminal = ratatui::init();
    let mut model = Model::default();

    // Loop
    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&model, frame))?;
        let mut current_message = handle_event()?;
        while current_message.is_some() {
            current_message = update(&mut model, current_message.unwrap());
        }
    }

    // Restore
    ratatui::restore();
    Ok(())
}

// Handles way keys are pressed
fn handle_event() -> color_eyre::Result<Option<Message>> {
    if let Event::Key(k) = event::read()? {
        return Ok(handle_key(k));
    }
    Ok(None)
}

// Gets the actual event associated with specific keys
fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

// Mutates the model or performs operations
fn update(model: &mut Model, message: Message) -> Option<Message> {
    match message {
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::Make { t, path } => todo!(),
        Message::Remove { path } => todo!(),
    }
    None
}

// Render func
fn view(model: &Model, frame: &mut Frame) {
    let layout = Layout::horizontal([
        Constraint::Fill(3),
        Constraint::Fill(4),
        Constraint::Fill(3),
    ])
    .split(frame.area());

    frame.render_widget(Block::bordered().border_set(border::ROUNDED), layout[0]);
    frame.render_widget(Block::bordered().border_set(border::ROUNDED), layout[1]);
    frame.render_widget(Block::bordered().border_set(border::ROUNDED), layout[2]);
}
