//! Session management for abduco.

use std::collections::HashMap;

/// Session state
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    /// Session is running
    Running,
    /// Session is stopped
    Stopped,
    /// Session is dead
    Dead,
}

/// Session type
#[derive(Debug, Clone, PartialEq)]
pub enum SessionType {
    /// Terminal session
    Terminal,
    /// Tmux session
    Tmux,
    /// Screen session
    Screen,
}

/// Session information
#[derive(Debug, Clone)]
pub struct Session {
    /// Session name
    pub name: String,
    /// Command to run
    pub command: String,
    /// Session state
    pub state: SessionState,
    /// Session type
    pub session_type: SessionType,
    /// Attached flag
    pub attached: bool,
    /// Created timestamp
    pub created_at: u64,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Working directory
    pub working_dir: String,
    /// Environment variables
    pub environment: HashMap<String, String>,
}

impl Session {
    /// Create new session
    pub fn new(name: String, command: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            name,
            command,
            state: SessionState::Running,
            session_type: SessionType::Terminal,
            attached: false,
            created_at: now,
            last_activity: now,
            working_dir: "/".into(),
            environment: HashMap::new(),
        }
    }
    
    /// Attach session
    pub fn attach(&mut self) {
        self.attached = true;
        self.update_activity();
    }
    
    /// Detach session
    pub fn detach(&mut self) {
        self.attached = false;
        self.update_activity();
    }
    
    /// Stop session
    pub fn stop(&mut self) {
        self.state = SessionState::Stopped;
        self.update_activity();
    }
    
    /// Kill session
    pub fn kill(&mut self) {
        self.state = SessionState::Dead;
        self.update_activity();
    }
    
    /// Update activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Set working directory
    pub fn set_working_dir(&mut self, dir: String) {
        self.working_dir = dir;
        self.update_activity();
    }
    
    /// Set environment variable
    pub fn set_env(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
        self.update_activity();
    }
    
    /// Get environment variable
    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.environment.get(key)
    }
    
    /// Get session duration
    pub fn get_duration(&self) -> u64 {
        let end_time = if self.state == SessionState::Dead {
            self.last_activity
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        };
        end_time.saturating_sub(self.created_at)
    }
    
    /// Check if session is active
    pub fn is_active(&self) -> bool {
        self.state == SessionState::Running
    }
    
    /// Check if session is attached
    pub fn is_attached(&self) -> bool {
        self.attached
    }
}
