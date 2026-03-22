use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
    let filename = format!("{}.txt", username);

    match self.cache.get_chat(&username) {
        None => {
            println!("get_history: {username} is not in the cache!");

            let mut chat_session = self.model.chat();

            if let Some(session) = file_library::load_chat_session_from_file(&filename) {
                chat_session = chat_session.with_session(session);
            }

            let history = chat_session
                .session()
                .expect("Failed to get session")
                .history()
                .iter()
                .map(|msg| format!("{:?}: {}", msg.role(), msg.content()))
                .collect::<Vec<String>>();

            self.cache.insert_chat(username, chat_session);

            history
        }
        Some(chat_session) => {
            println!("get_history: {username} is in the cache! Nice!");

            chat_session
                .session()
                .expect("Failed to get session")
                .history()
                .iter()
                .map(|msg| format!("{:?}: {}", msg.role(), msg.content()))
                .collect::<Vec<String>>()
        }
    }
}
}

