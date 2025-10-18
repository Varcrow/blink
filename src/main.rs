use std::env::current_dir;

use crate::stfm::app::App;
mod stfm;

fn main() -> color_eyre::Result<()> {
    App::new(current_dir()?.to_path_buf()).run()
}
