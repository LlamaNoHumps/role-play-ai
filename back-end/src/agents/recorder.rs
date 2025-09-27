use super::RetryConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;

#[derive(Clone)]
pub struct Recorder {
    http_client: reqwest::Client,
    retry_config: RetryConfig,
}

impl Recorder {
    pub fn new(ai_api_key: &str) -> Self {
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", ai_api_key).parse().unwrap(),
            );
            headers
        };

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            http_client,
            retry_config: RetryConfig::default(),
        }
    }

    pub async fn asr(&self, voice_url: &str) -> Result<String> {
        const URL: &str = "https://openai.qiniu.com/v1/voice/asr";

        let data = RequestParams {
            model: "asr".to_string(),
            audio: AudioInfo {
                format: "mp3".to_string(),
                url: voice_url.to_string(),
            },
        };

        let mut last_error = None;

        for attempt in 0..self.retry_config.max_retries {
            match self.asr_attempt(&data, URL).await {
                Ok(text) => {
                    if attempt > 0 {
                        tracing::info!(
                            "ASR request succeeded on attempt {} for URL: {}",
                            attempt + 1,
                            voice_url
                        );
                    }
                    return Ok(text);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_config.max_retries - 1 {
                        let delay =
                            Duration::from_millis(self.retry_config.base_delay_ms * (1 << attempt)); // 指数退避
                        tracing::warn!(
                            "ASR request failed on attempt {}/{}, retrying in {:?}: {}",
                            attempt + 1,
                            self.retry_config.max_retries,
                            delay,
                            last_error.as_ref().unwrap()
                        );
                        sleep(delay).await;
                    }
                }
            }
        }

        tracing::error!(
            "ASR request failed after {} attempts for URL {}: {}",
            self.retry_config.max_retries,
            voice_url,
            last_error.as_ref().unwrap()
        );
        Err(last_error.unwrap())
    }

    async fn asr_attempt(&self, data: &RequestParams, url: &str) -> Result<String> {
        let res = self.http_client.post(url).json(data).send().await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!("ASR API returned status: {}", res.status()));
        }

        let body = res.text().await?;
        let res = serde_json::from_str::<Response>(&body)?;

        Ok(res.data.result.text)
    }
}

#[derive(Serialize)]
struct RequestParams {
    model: String,
    audio: AudioInfo,
}

#[derive(Serialize)]
struct AudioInfo {
    format: String,
    url: String,
}

#[derive(Deserialize)]
struct Response {
    data: ResponseData,
    #[serde(flatten)]
    _others: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct ResponseData {
    result: ResponseDataResult,
    #[serde(flatten)]
    _others: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct ResponseDataResult {
    text: String,
    #[serde(flatten)]
    _others: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_asr() {
        let env = get_env();

        let recorder = Recorder::new(&env.qiniu_ai_api_key);
        let audio_data = recorder.asr("http://t2zfj0z3c.hd-bkt.clouddn.com/audio_e01b5f19-0bd9-4510-a580-fd6c44d14d53.mp3").await.unwrap();

        println!("{}", audio_data);
    }
}
