#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    pub event_type: KeyEventType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyEventType {
    Inception,
    Rotation,
}

pub struct KeyEventLog {
    pub events: Vec<KeyEvent>,
}

impl KeyEventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: KeyEvent) {
        self.events.push(event);
    }
}
