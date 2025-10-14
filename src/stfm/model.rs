#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
