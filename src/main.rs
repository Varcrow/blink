use crate::stfm::app::App;

mod stfm;

fn main() -> color_eyre::Result<()> {
    App::default().run()
}
