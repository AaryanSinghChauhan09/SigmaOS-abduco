//! SigmaOS-abduco — comprehensive terminal session manager.
//!
//! Provides terminal session management with session persistence,
//! attachment/detachment, and D-Bus integration for session control.

pub mod session;
pub mod manager;
pub mod options;

pub use session::{Session, SessionState, SessionType};
pub use manager::{SessionManager, SessionManagerOptions};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let session = Session::new("test".into(), "bash".into());
        assert_eq!(session.name, "test");
        assert_eq!(session.command, "bash");
        assert_eq!(session.state, SessionState::Running);
    }
    
    #[test]
    fn test_session_manager_creation() {
        let mut manager = SessionManager::new();
        assert!(manager.list_sessions().is_empty());
    }
    
    #[test]
    fn test_start_session() {
        let mut manager = SessionManager::new();
        assert!(manager.start_session("test".into(), "bash".into()));
        assert!(!manager.start_session("test".into(), "bash".into())); // Duplicate
        assert_eq!(manager.list_sessions().len(), 1);
    }
    
    #[test]
    fn test_attach_session() {
        let mut manager = SessionManager::new();
        manager.start_session("test".into(), "bash".into());
        
        assert!(manager.attach_session("test"));
        let session = manager.get_session("test").unwrap();
        assert!(session.attached);
    }
    
    #[test]
    fn test_detach_session() {
        let mut manager = SessionManager::new();
        manager.start_session("test".into(), "bash".into());
        manager.attach_session("test");
        
        assert!(manager.detach_session("test"));
        let session = manager.get_session("test").unwrap();
        assert!(!session.attached);
    }
    
    #[test]
    fn test_kill_session() {
        let mut manager = SessionManager::new();
        manager.start_session("test".into(), "bash".into());
        
        assert!(manager.kill_session("test"));
        assert!(manager.get_session("test").is_none());
    }
    
    #[test]
    fn test_session_types() {
        let types = vec![
            SessionType::Terminal,
            SessionType::Tmux,
            SessionType::Screen,
        ];
        assert_eq!(types.len(), 3);
    }
    
    #[test]
    fn test_session_states() {
        let states = vec![
            SessionState::Running,
            SessionState::Stopped,
            SessionState::Dead,
        ];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_list_sessions() {
        let mut manager = SessionManager::new();
        manager.start_session("test1".into(), "bash".into());
        manager.start_session("test2".into(), "zsh".into());
        
        let sessions = manager.list_sessions();
        assert_eq!(sessions.len(), 2);
    }
    
    #[test]
    fn test_get_session() {
        let mut manager = SessionManager::new();
        manager.start_session("test".into(), "bash".into());
        
        let session = manager.get_session("test");
        assert!(session.is_some());
        assert_eq!(session.unwrap().command, "bash");
    }
    
    #[test]
    fn test_session_statistics() {
        let mut manager = SessionManager::new();
        manager.start_session("test1".into(), "bash".into());
        manager.start_session("test2".into(), "zsh".into());
        manager.attach_session("test1");
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_sessions, 2);
        assert_eq!(stats.attached_sessions, 1);
        assert_eq!(stats.running_sessions, 2);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = SessionManager::new();
        manager.start_session("test".into(), "bash".into());
        manager.attach_session("test");
        
        let export = manager.export_data();
        
        let mut manager2 = SessionManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_sessions().len(), 1);
        assert!(manager2.get_session("test").unwrap().attached);
    }
}
