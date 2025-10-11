#[derive(Debug, Default, PartialEq, Eq)]
pub enum  ModelState {
    #[default]
    Running, 
    Complete,
    Quit,
}
