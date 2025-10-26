use crate::blink::app::App;
use std::env::current_dir;
mod blink;

fn main() -> color_eyre::Result<()> {
   App::new(current_dir()?.to_path_buf())?.run()
}
