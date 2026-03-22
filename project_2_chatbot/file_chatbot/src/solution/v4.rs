use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
    let filename = &format!("{}.txt", username);
    let filename = &format!("{}.txt", username);

    let mut chat_session: Chat<Llama> = self.model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");
    let mut chat_session: Chat<Llama> = self.model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");

    // Load previous session
    match file_library::load_chat_session_from_file(&filename) {
        None => {}
        Some(session) => {
            chat_session = chat_session.with_session(session);
        }
    }

    // Get response
    let response = chat_session
        .add_message(message)
        .await
        .expect("Failed to generate response");

    // Get session
    let session = chat_session
        .session()
        .expect("Failed to get session");

    // Save session
    file_library::save_chat_session_to_file(&filename, &session);

    return response;
}

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                // TODO: what should happen here?
                return Vec::new();
            }
        }
    }
}
