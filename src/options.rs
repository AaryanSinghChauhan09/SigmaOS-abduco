//! Command-line options for abduco.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Attach to session
    Attach,
    /// Start new session
    Start,
    /// List sessions
    List,
    /// Kill session
    Kill,
    /// Detach from session
    Detach,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Session name
    pub session_name: String,
    /// Command to run
    pub command: String,
    /// Working directory
    pub working_dir: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Start,
            session_name: String::new(),
            command: String::new(),
            working_dir: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
