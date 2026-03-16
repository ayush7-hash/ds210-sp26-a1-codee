use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    session: Chat<Llama>,
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        ChatbotV2 {
            session: model
                .chat()
                .with_system_prompt("The assistant will act like a pirate"),
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response = self.session.add_message(message).await;

        return match response {
            Ok(msg) => msg,
            Err(_) => String::from("Error generating response"),
        }
        // Add your code for chatting with the agent while keeping conversation history here.
    }
}