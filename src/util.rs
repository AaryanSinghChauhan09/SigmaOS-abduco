//! abduco model — session manager with D-Bus support.

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub command: String,
    pub attached: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self, name: &str, command: &str) -> bool {
        if !self.sessions.iter().any(|s| s.name == name) {
            self.sessions.push(Session { name: name.into(), command: command.into(), attached: false });
            true
        } else { false }
    }

    pub fn attach(&mut self, name: &str) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.name == name) {
            s.attached = true;
            true
        } else { false }
    }

    pub fn detach(&mut self, name: &str) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.name == name) {
            s.attached = false;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_attach_detach() {
        let mut a = Model::new();
        assert!(a.start("main", "bash"));
        assert!(a.attach("main"));
        assert!(a.detach("main"));
    }
}
