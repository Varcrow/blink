pub enum Log {
    Info{message: String},
    Warning{message: String},
    Error{message: String},
}

pub struct LogManager {
    pub session_logs: Vec<Log>,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            session_logs: Vec::new(),
        }
    }

    pub fn add_log(&mut self, log: Log) {
        self.session_logs.push(log);
    }
}
