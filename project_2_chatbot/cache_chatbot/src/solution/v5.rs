use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        ChatbotV5 {
            model,
            cache: Cache::new(3),
        }
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename: &String = &format! ("{}.txt", username);
        let cached_chat: Option<&mut Chat<Llama>> = self. cache.get_chat (&username);

        match cached_chat {
            None => {
                println! ("chat_with_user: {username} is not in the cache!");
                // since the convo is not in the cache, we can check if its in the file
                // try to load previous conversation
                let mut chat: Chat<Llama> =
                if let Some(session) = file_library:: load_chat_session_from_file(filename) {
                    println! ("Loaded {username) from file");
                // rebuild the chat
                    self.model.chat().with_session(session)
                }
                // no file found, new user
                else {
                    println! ("Creating new chat for {username}");
                    self.model.chat()
                };
                // add the message to the conversation, and comes up with a response
                let response: String = chat.add_message (message).await.unwrap().to_string();
                // save the session to the file
                // &chat.session.unwrap() here because mismatched types
                file_library::save_chat_session_to_file(filename,  &chat.session().unwrap());
                //put into the cache
                self.cache.insert_chat(username, chat);
                return response;
            }
        Some (chat_session) => {
            println! ("chat_with_user: {username} is in the cache! Nice!");
            // the chat is already in memory
            // add the message to the conversation, and comes up with a response
            let response: String = chat_session.add_message(message).await.unwrap().to_string();
            // save the convo to the file
            // save the session to the file
            // &chat.session.unwrap() here because mismatched types
            file_library::save_chat_session_to_file(filename, &chat_session.session().unwrap());
            return response;
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

        self.cache.get_history(&username)

    }
}
}

