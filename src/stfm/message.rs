#[derive(PartialEq)]
pub enum Message {
    Make { t: String, path: String },
    Remove { path: String },
    Quit,
}
