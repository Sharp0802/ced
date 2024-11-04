
pub struct Global {
    current_file: String,
    shutdown: bool,
}

impl Global {
    pub fn new() -> Self {
        Self {
            current_file: String::new(),
            shutdown: false,
        }
    }

    pub fn current_file(&self) -> &str {
        &self.current_file
    }

    pub fn shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn set_shutdown(&mut self) {
        self.shutdown = true;
    }
}
