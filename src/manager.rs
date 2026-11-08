//! Session manager for abduco.

use std::collections::HashMap;
use crate::session::{Session, SessionState, SessionType};

/// Session manager options
#[derive(Debug, Clone)]
pub struct SessionManagerOptions {
    /// Maximum sessions
    pub max_sessions: usize,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Auto-attach on start
    pub auto_attach: bool,
    /// Enable D-Bus integration
    pub dbus_enabled: bool,
}

impl Default for SessionManagerOptions {
    fn default() -> Self {
        Self {
            max_sessions: 100,
            session_timeout: 3600,
            auto_attach: false, // Changed to false - sessions start detached
            dbus_enabled: true,
        }
    }
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStatistics {
    /// Total sessions
    pub total_sessions: usize,
    /// Running sessions
    pub running_sessions: usize,
    /// Stopped sessions
    pub stopped_sessions: usize,
    /// Dead sessions
    pub dead_sessions: usize,
    /// Attached sessions
    pub attached_sessions: usize,
    /// Detached sessions
    pub detached_sessions: usize,
}

/// Session manager
#[derive(Debug, Clone)]
pub struct SessionManager {
    /// Sessions
    sessions: HashMap<String, Session>,
    /// Options
    options: SessionManagerOptions,
    /// Statistics
    statistics: SessionStatistics,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
            options: SessionManagerOptions::default(),
            statistics: SessionStatistics {
                total_sessions: 0,
                running_sessions: 0,
                stopped_sessions: 0,
                dead_sessions: 0,
                attached_sessions: 0,
                detached_sessions: 0,
            },
        }
    }
}

impl SessionManager {
    /// Create new session manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create session manager with options
    pub fn with_options(options: SessionManagerOptions) -> Self {
        let mut manager = Self::default();
        manager.options = options;
        manager
    }
    
    /// Start new session
    pub fn start_session(&mut self, name: String, command: String) -> bool {
        // Check session limit
        if self.sessions.len() >= self.options.max_sessions {
            return false;
        }
        
        // Check if session already exists
        if self.sessions.contains_key(&name) {
            return false;
        }
        
        let mut session = Session::new(name.clone(), command);
        
        // Auto-attach if enabled
        if self.options.auto_attach {
            session.attach();
        }
        
        self.sessions.insert(name, session);
        self.update_statistics();
        true
    }
    
    /// Attach to session
    pub fn attach_session(&mut self, name: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(name) {
            session.attach();
            self.update_statistics();
            true
        } else {
            false
        }
    }
    
    /// Detach from session
    pub fn detach_session(&mut self, name: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(name) {
            session.detach();
            self.update_statistics();
            true
        } else {
            false
        }
    }
    
    /// Stop session
    pub fn stop_session(&mut self, name: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(name) {
            session.stop();
            self.update_statistics();
            true
        } else {
            false
        }
    }
    
    /// Kill session
    pub fn kill_session(&mut self, name: &str) -> bool {
        // Kill the session first
        if let Some(session) = self.sessions.get_mut(name) {
            session.kill();
        }
        // Then remove it
        if self.sessions.remove(name).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Remove session
    pub fn remove_session(&mut self, name: &str) -> bool {
        if self.sessions.remove(name).is_some() {
            self.update_statistics();
            true
        } else {
            false
        }
    }
    
    /// Get session
    pub fn get_session(&self, name: &str) -> Option<&Session> {
        self.sessions.get(name)
    }
    
    /// Get session mutably
    pub fn get_session_mut(&mut self, name: &str) -> Option<&mut Session> {
        self.sessions.get_mut(name)
    }
    
    /// List all sessions
    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.values().collect()
    }
    
    /// List running sessions
    pub fn list_running_sessions(&self) -> Vec<&Session> {
        self.sessions.values()
            .filter(|s| s.state == SessionState::Running)
            .collect()
    }
    
    /// List attached sessions
    pub fn list_attached_sessions(&self) -> Vec<&Session> {
        self.sessions.values()
            .filter(|s| s.attached)
            .collect()
    }
    
    /// Update session working directory
    pub fn set_session_working_dir(&mut self, name: &str, dir: String) -> bool {
        if let Some(session) = self.sessions.get_mut(name) {
            session.set_working_dir(dir);
            true
        } else {
            false
        }
    }
    
    /// Set session environment variable
    pub fn set_session_env(&mut self, name: &str, key: String, value: String) -> bool {
        if let Some(session) = self.sessions.get_mut(name) {
            session.set_env(key, value);
            true
        } else {
            false
        }
    }
    
    /// Clean up dead sessions
    pub fn cleanup_dead_sessions(&mut self) -> usize {
        let dead_sessions: Vec<String> = self.sessions.values()
            .filter(|s| s.state == SessionState::Dead)
            .map(|s| s.name.clone())
            .collect();
        
        let count = dead_sessions.len();
        for name in dead_sessions {
            self.sessions.remove(&name);
        }
        
        self.update_statistics();
        count
    }
    
    /// Clean up timed out sessions
    pub fn cleanup_timed_out_sessions(&mut self) -> usize {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let timed_out: Vec<String> = self.sessions.values()
            .filter(|s| {
                now - s.last_activity > self.options.session_timeout &&
                s.state == SessionState::Running
            })
            .map(|s| s.name.clone())
            .collect();
        
        let count = timed_out.len();
        for name in timed_out {
            if let Some(session) = self.sessions.get_mut(&name) {
                session.stop();
            }
        }
        
        self.update_statistics();
        count
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_sessions = self.sessions.len();
        self.statistics.running_sessions = self.sessions.values()
            .filter(|s| s.state == SessionState::Running)
            .count();
        self.statistics.stopped_sessions = self.sessions.values()
            .filter(|s| s.state == SessionState::Stopped)
            .count();
        self.statistics.dead_sessions = self.sessions.values()
            .filter(|s| s.state == SessionState::Dead)
            .count();
        self.statistics.attached_sessions = self.sessions.values()
            .filter(|s| s.attached)
            .count();
        self.statistics.detached_sessions = self.statistics.total_sessions - self.statistics.attached_sessions;
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &SessionStatistics {
        &self.statistics
    }
    
    /// Export session data
    pub fn export_data(&self) -> Vec<Session> {
        self.sessions.values().cloned().collect()
    }
    
    /// Import session data
    pub fn import_data(&mut self, sessions: Vec<Session>) {
        self.sessions.clear();
        for session in sessions {
            self.sessions.insert(session.name.clone(), session);
        }
        self.update_statistics();
    }
}
