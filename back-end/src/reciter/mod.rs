use crate::storage::StorageClient;
use anyhow::Result;
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

pub struct Reciter {
    storage_client: Arc<StorageClient>,
    http_client: reqwest::Client,
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
        }
    }

    pub async fn tts(&self, text: &str, gender: Gender, voice_type: VoiceType) -> Result<Vec<u8>> {
        const URL: &str = "https://openai.qiniu.com/v1/voice/tts";

        let voice_type = match (gender, voice_type) {
            (Gender::Male, VoiceType::Mature) => "qiniu_zh_male_ybxknjs",
            (Gender::Male, VoiceType::Young) => "qiniu_zh_male_hlsnkk",
            (Gender::Female, VoiceType::Mature) => "qiniu_zh_female_zxjxnjs",
            (Gender::Female, VoiceType::Young) => "qiniu_zh_female_tmjxxy",
        };

        let data = RequestParams {
            audio: AudioInfo {
                voice_type: voice_type.to_string(),
                encoding: "mp3".to_string(),
                speed_ratio: Some(1.5),
            },
            request: RequestText {
                text: text.to_string(),
            },
        };

        let res = self.http_client.post(URL).json(&data).send().await?;

        let body = res.text().await?;
        let res = serde_json::from_str::<Response>(&body)?;

        let audio_data = general_purpose::STANDARD.decode(res.data)?;

        Ok(audio_data)
    }

    pub async fn upload_audio(&self, data: Vec<u8>) -> Result<String> {
        let file_name = format!("audio_{}.mp3", uuid::Uuid::new_v4());
        let object_info = self.storage_client.upload_object(&file_name, data).await?;

        Ok(object_info.key)
    }
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

pub enum Gender {
    Male,
    Female,
}

pub enum VoiceType {
    Mature,
    Young,
}

#[derive(Deserialize)]
struct Response {
    reqid: String,
    operation: String,
    sequence: i8,
    data: String,
    addition: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_tts() {
        let env = get_env();

        let storage_client = StorageClient::new(&env.qiniu_access_key, &env.qiniu_secret_key);
        storage_client.init_bucket().await.unwrap();
        let reciter = Reciter::new(Arc::new(storage_client), &env.qiniu_ai_api_key);
        let audio_data = reciter
            .tts(
                "你好，欢迎使用七牛云语音合成服务",
                Gender::Female,
                VoiceType::Mature,
            )
            .await
            .unwrap();

        reciter.upload_audio(audio_data).await.unwrap();
    }
}
