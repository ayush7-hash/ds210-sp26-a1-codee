use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        let response = chat_session.add_message(message).await;

        match response {
            Ok(reply) => reply,
            Err(_) => String::from("Sorry, I ran into an error while responding."),
        }
        println!("{message}");
        let asynchronous_output: ChatResponseBuilder<'_, Llama> = chat_session.add_message(message);
        let output: String = asynchronous_output.await.unwrap();
        output.to_string()
       
        // You need to add your code here
        // You must find a way to add the given message to the chat_session!
        // consider https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.add_message
        // Hint: make sure you transform/extract the response message as a **String**.

        //return String::from("Hello, I am not a bot (yet)!");
    }
}