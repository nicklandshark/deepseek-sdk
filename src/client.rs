use crate::errors::Result;
use crate::schema::request::ChatCompletionsRequest;
use crate::schema::response::ChatCompletionsResponse;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, ClientBuilder};

#[derive(Default)]
pub struct Deepseek {
    inner: Client,
}

impl Deepseek {
    const BASE_URL: &'static str = "https://api.deepseek.com";

    /// Requires environment variable `DEEPSEEK_API_KEY` to be set
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        let api_key = std::env::var("DEEPSEEK_API_KEY").expect("env `DEEPSEEK_API_KEY` to be set");
        let bearer_api_key = format!("Bearer {}", api_key);
        let v = HeaderValue::from_str(&bearer_api_key).expect("env `DEEPSEEK_API_KEY` to be ASCII");
        headers.append(AUTHORIZATION, v);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { inner: client }
    }

    pub async fn execute(&self, req: ChatCompletionsRequest) -> Result<ChatCompletionsResponse> {
        let chat_endpoint = format!("{}/chat/completions", Self::BASE_URL);

        let resp = self
            .inner
            .post(chat_endpoint)
            .json::<ChatCompletionsRequest>(&req)
            .send()
            .await?;

        let completion = resp.json::<ChatCompletionsResponse>().await?;

        Ok(completion)
    }
}
