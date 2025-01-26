use crate::monitor::Monitor;

#[derive(Clone)]
pub struct UcuiState {
    pub monitor: Monitor,
}

impl UcuiState {
    pub fn new() -> Self {
        Self {
            monitor: Monitor::new(),
        }
    }
}
