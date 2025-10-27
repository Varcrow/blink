use crate::blink::app::App;
use ratatui::{Frame, crossterm::event::KeyCode};

pub trait State {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State>;
    fn render(&self, app: &App, frame: &mut Frame);
}
