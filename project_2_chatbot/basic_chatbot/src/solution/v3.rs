use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // Store the model so new chat sessions can be created
    model: Llama,

    // Map each username to their own chat session
    chats: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        ChatbotV3 {
            model,
            chats: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // If this is the first message from the user, create a new chat session
        if !self.chats.contains_key(&username) {
            let chat = self.model.chat();
            self.chats.insert(username.clone(), chat);
        }

        // Retrieve the correct chat session for this user
        let chat = self.chats.get_mut(&username).unwrap();

        // Add the user's message and generate a response
        let response = chat.add_message(message).await.unwrap();

        response
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Retrieve the chat history for the given user
        if let Some(chat) = self.chats.get(&username) {
            if let Ok(session) = chat.session() {
                return session
                    .history()
                    .iter()
                    .map(|msg| format!("{:?}: {}", msg.role(), msg.content()))
                    .collect();
            }
        }

        // Return empty history if the user has no chat session yet
        Vec::new()
    }
}