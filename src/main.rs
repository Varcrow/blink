use crate::stfm::app::App;

mod stfm;

fn main() -> color_eyre::Result<()> {
    {
        let mut this = App::default();
        while this.running {
            terminal.draw(|frame| this.render(frame))?;

            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => this.running = false,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
