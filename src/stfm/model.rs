#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub directories: Vec<Vec<String>>, 
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
