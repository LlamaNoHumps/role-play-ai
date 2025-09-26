use super::RetryConfig;
use crate::{database::models::roles::Gender, storage::StorageClient};
use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::sleep;

#[derive(Clone)]
pub struct Reciter {
    storage_client: Arc<StorageClient>,
    http_client: reqwest::Client,
    retry_config: RetryConfig,
}

impl Reciter {
    pub fn new(storage_client: Arc<StorageClient>, ai_api_key: &str) -> Self {
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
            storage_client,
            http_client,
            retry_config: RetryConfig::default(),
        }
    }

    pub async fn tts(&self, text: &str, voice_type: &str) -> Result<Vec<u8>> {
        const URL: &str = "https://openai.qiniu.com/v1/voice/tts";

        let data = RequestParams {
            audio: AudioInfo {
                voice_type: voice_type.to_string(),
                encoding: "mp3".to_string(),
                speed_ratio: Some(1.0),
            },
            request: RequestText {
                text: text.to_string(),
            },
        };

        let mut last_error = None;

        for attempt in 0..self.retry_config.max_retries {
            match self.tts_attempt(&data, URL).await {
                Ok(audio_data) => {
                    if attempt > 0 {
                        tracing::info!(
                            "TTS request succeeded on attempt {} for text: {}",
                            attempt + 1,
                            &text[..text.len().min(50)]
                        );
                    }
                    return Ok(audio_data);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_config.max_retries - 1 {
                        let delay =
                            Duration::from_millis(self.retry_config.base_delay_ms * (1 << attempt)); // 指数退避
                        tracing::warn!(
                            "TTS request failed on attempt {}/{}, retrying in {:?}: {}",
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
            "TTS request failed after {} attempts: {}",
            self.retry_config.max_retries,
            last_error.as_ref().unwrap()
        );
        Err(last_error.unwrap())
    }

    async fn tts_attempt(&self, data: &RequestParams, url: &str) -> Result<Vec<u8>> {
        let res = self.http_client.post(url).json(data).send().await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!("TTS API returned status: {}", res.status()));
        }

        let body = res.text().await?;
        let res = serde_json::from_str::<Response>(&body)?;

        let audio_data = general_purpose::STANDARD.decode(res.data)?;

        Ok(audio_data)
    }

    pub async fn upload_audio(&self, data: Vec<u8>) -> Result<String> {
        let file_name = format!("audio_{}.mp3", uuid::Uuid::new_v4());
        let object_info = self.storage_client.upload_object(&file_name, data).await?;

        let url = self.storage_client.get_object_url(&object_info.key);

        Ok(url)
    }

    pub async fn fetch_voice_map(&self) -> Result<HashMap<String, VoiceInfo>> {
        const URL: &str = "https://openai.qiniu.com/v1/voice/list";

        let mut last_error = None;
        for attempt in 0..self.retry_config.max_retries {
            match self.fetch_voice_list_attempt(URL).await {
                Ok(voice_list) => {
                    if attempt > 0 {
                        tracing::info!("Fetch voice list succeeded on attempt {}", attempt + 1,);
                    }

                    let mut voice_map = HashMap::new();
                    for v in voice_list {
                        voice_map.insert(
                            v.voice_type.clone(),
                            VoiceInfo {
                                voice_name: v.voice_name.clone(),
                                gender: match v.voice_type.as_str() {
                                    vt if vt.contains("male") => Gender::Male,
                                    vt if vt.contains("female") => Gender::Female,
                                    _ => return Err(anyhow!("Unknown gender")),
                                },
                                category: v.category.clone(),
                            },
                        );
                    }

                    return Ok(voice_map);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_config.max_retries - 1 {
                        let delay =
                            Duration::from_millis(self.retry_config.base_delay_ms * (1 << attempt)); // 指数退避
                        tracing::warn!(
                            "Fetch voice list failed on attempt {}/{}, retrying in {:?}: {}",
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
            "Fetch voice list failed after {} attempts: {}",
            self.retry_config.max_retries,
            last_error.as_ref().unwrap()
        );
        Err(last_error.unwrap())
    }

    async fn fetch_voice_list_attempt(&self, url: &str) -> Result<Vec<VoiceItem>> {
        let res = self.http_client.get(url).send().await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!(
                "Fetch voice list API returned status: {}",
                res.status()
            ));
        }

        let body = res.text().await?;
        let res = serde_json::from_str::<Vec<VoiceItem>>(&body)?;

        Ok(res)
    }
}

// 去除文本括号中的内容
pub fn remove_brackets(text: &str) -> String {
    let mut result = String::new();
    let mut skip = 0;

    for c in text.chars() {
        match c {
            '(' | '（' => skip += 1,
            ')' | '）' => {
                if skip > 0 {
                    skip -= 1;
                }
            }
            _ => {
                if skip == 0 {
                    result.push(c);
                }
            }
        }
    }

    result
}

#[derive(Serialize)]
struct RequestParams {
    audio: AudioInfo,
    request: RequestText,
}

#[derive(Serialize)]
struct AudioInfo {
    voice_type: String,
    encoding: String,
    speed_ratio: Option<f32>,
}

#[derive(Serialize)]
struct RequestText {
    text: String,
}

#[derive(Deserialize)]
struct Response {
    data: String,
    #[serde(flatten)]
    _others: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
pub struct VoiceItem {
    pub voice_name: String,
    pub voice_type: String,
    pub category: String,
    #[serde(flatten)]
    _others: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
pub struct VoiceInfo {
    pub voice_name: String,
    pub gender: Gender,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_tts() {
        let env = get_env();

        let mut storage_client = StorageClient::new(&env.qiniu_access_key, &env.qiniu_secret_key);
        storage_client.init_bucket().await.unwrap();
        let reciter = Reciter::new(Arc::new(storage_client), &env.qiniu_ai_api_key);
        let audio_data = reciter
            .tts("你好，欢迎使用七牛云语音合成服务", "qiniu_zh_female_cxjxgw")
            .await
            .unwrap();

        reciter.upload_audio(audio_data).await.unwrap();
    }
}
