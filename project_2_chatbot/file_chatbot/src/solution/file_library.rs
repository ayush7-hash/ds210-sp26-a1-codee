use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    let bytes = session.to_bytes();
    fs::write(filename, bytes).expect("Failed to write chat session to file");
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    let bytes = match std::fs::read(filename) {
        Ok(b) => b,
        Err(_) => return None,
    };

    let session = match LlamaChatSession::from_bytes(&bytes) {
        Ok(s) => s,
        Err(_) => return None,
    };

    return Some(session);
    // look at fs::read(...)
    // also look at LlamaChatSession::from_bytes(...)
    unimplemented!("Loading chat session from file {filename}");
}
