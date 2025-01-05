mod client;

pub mod errors;
pub mod schema;

pub use client::Deepseek;

pub mod prelude {
    pub use crate::client::Deepseek;
    pub use crate::schema::request::{ChatCompletionsRequest, ChatCompletionsRequestBuilder};
    pub use crate::schema::response::ChatCompletionsResponse;
    pub use crate::schema::{Message, MessageBuilder};
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::schema::tool::*;

    use dotenv::dotenv;
    use eyre::Result;

    #[tokio::test]
    async fn request_works() -> Result<()> {
        // Example of multi turn conversation where the assistant counts by 1
        dotenv().ok();
        let api = Deepseek::new();

        // First start with a few shot prompt to increase instruction following
        let mut messages: Vec<Message> = vec![
            Message::builder()
                .role("system")?
                .content("You are a helpful counting assistant. Be brief. Reply only with numbers.")
                .build()?,
            Message::builder()
                .role("user")?
                .content("Your objective is to count by 1. Start at 0")
                .build()?,
            Message::builder().role("assistant")?.content("0").build()?,
            Message::builder()
                .role("user")?
                .content("Count by 1!")
                .build()?,
            Message::builder().role("assistant")?.content("1").build()?,
            Message::builder()
                .role("user")?
                .content("Count by 1!")
                .build()?,
            Message::builder().role("assistant")?.content("2").build()?,
        ];

        // Then each iteration feed it previous response to the new request
        loop {
            // Counting prompt
            messages.push(
                Message::builder()
                    .role("user")?
                    .content("Count by 1!")
                    .build()?,
            );

            let request = ChatCompletionsRequest::builder()
                .model("deepseek-chat")
                .messages(messages.clone())
                .build()?;
            let resp = api.execute(request).await?;
            let content = &resp.choices[0].message.content;

            // For debugging (cargo test -- --nocapture)
            println!("Assistant: {}", content);

            if content.contains('5') {
                break;
            }

            // Feed the current response for the next iteration's request
            messages.push(
                Message::builder()
                    .role("assistant")?
                    .content(content)
                    .build()?,
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn json_response_format_works() -> Result<()> {
        dotenv().ok();
        let api = Deepseek::new();

        // Response Format works best with an accompanying system prompt
        let mut messages: Vec<Message> = vec![
            Message::builder()
                .role("system")?
                .content(r#"
    The user will provide some exam text. Please parse the "question" and "answer" and output them in JSON format.

    EXAMPLE INPUT:
    Which is the highest mountain in the world? Mount Everest.

    EXAMPLE JSON OUTPUT:
    {
        "question": "Which is the highest mountain in the world?",
        "answer": "Mount Everest"
    }
                    "#)
                .build()?
        ];

        let prompt = "What is the longest river in the world? The Nile River";

        messages.push(Message::builder().role("user")?.content(prompt).build()?);

        let request = ChatCompletionsRequest::builder()
            .model("deepseek-chat")
            .messages(messages)
            .json()
            .build()?;
        let resp = api.execute(request).await?;
        let content = &resp.choices[0].message.content;

        // For debugging (cargo test -- --nocapture)
        println!("Assistant: {}", content);

        // Assert that content is json
        #[derive(serde::Deserialize)]
        struct ExpectedResponse {
            question: String,
            answer: String,
        }

        let json = serde_json::from_str::<ExpectedResponse>(content)?;
        assert_eq!(json.question, "What is the longest river in the world?");
        assert_eq!(json.answer, "The Nile River");

        Ok(())
    }

    #[tokio::test]
    async fn tool_calling_works() -> eyre::Result<()> {
        dotenv::dotenv().ok();
        let api = Deepseek::new();

        // Create mock function definition for JSON validator
        let validate_json_function = Tool {
            tool_type: "function".into(),
            function: ToolFunction {
                name: "validate_json".into(),
                description: Some("Validate JSON string against a JSON schema.".into()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "json_string": {
                            "type": "string",
                            "description": "The JSON string to validate"
                        },
                        "schema": {
                            "type": "string",
                            "description": "The JSON schema to validate against"
                        },
                    },
                    "required": ["json_string", "schema"],
                    "additionalProperties": false
                })),
            },
        };

        let messages = vec![
            Message::builder()
                .role("system")?
                .content("You are a helpful JSON validation assistant. Use the validate_json function to validate JSON when requested.")
                .build()?,
            Message::builder()
                .role("user")?
                .content(r#"Can you validate this JSON against a schema that expects a name and age?

                    {
                        "name": "John",
                        "age": 30
                    }"#)
                .build()?,
        ];

        let request = ChatCompletionsRequest::builder()
            .model("deepseek-chat")
            .messages(messages.clone())
            .tools(vec![validate_json_function])
            .build()?;

        let resp = api.execute(request).await?;

        // Get first tool call
        let tool_call = &resp.choices[0]
            .message
            .tool_calls
            .as_ref()
            .expect("Expected tool call")[0];

        // Check that correct function was called
        assert_eq!(tool_call.function.name, "validate_json");

        // Assert the schema is correct
        let args: serde_json::Value = serde_json::from_str(&tool_call.function.arguments)?;
        let schema_value =
            serde_json::from_str::<serde_json::Value>(args["schema"].as_str().unwrap())?;
        assert_eq!(schema_value["type"], "object");
        assert!(schema_value["properties"]["name"]["type"] == "string");
        assert!(schema_value["properties"]["age"]["type"] == "integer");

        // Now simulate the tool call response
        let mut messages = messages;
        messages.push(resp.choices[0].message.clone());

        // Add tool response
        messages.push(
            Message::builder()
                .role("tool")?
                .content("JSON is valid against schema")
                .tool_call_id(tool_call.id.clone())
                .build()?,
        );

        let request = ChatCompletionsRequest::builder()
            .model("deepseek-chat")
            .messages(messages)
            .build()?;

        let resp = api.execute(request).await?;

        let content = &resp.choices[0].message.content;

        // For debugging (cargo test -- --nocapture)
        println!("Assistant: {}", content);

        assert!(
            content.contains("valid"),
            "Expected response to mention JSON validity"
        );

        Ok(())
    }
}
