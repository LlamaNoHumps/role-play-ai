mod utils;

use crate::env::utils::get_env_value_option;
use std::sync::OnceLock;
use utils::get_env_value;

pub static ENV: OnceLock<Env> = OnceLock::new();

pub struct Env {
    pub port: u16,
    pub tracing_level: String,
    pub qiniu_access_key: String,
    pub qiniu_secret_key: String,
    pub qiniu_ai_api_key: String,
    pub mysql_username: String,
    pub mysql_password: String,
    pub mysql_endpoint: String,
    pub qiniu_llm_model: String,
    pub qiniu_llm_thinking_model: String,
}

impl Env {
    pub fn new() -> Self {
        let port = get_env_value_option("PORT", 8080);
        let tracing_level = get_env_value_option("TRACING_LEVEL", "info".to_string());
        let qiniu_access_key =
            get_env_value("QINIU_ACCESS_KEY").expect("QINIU_ACCESS_KEY must be set");
        let qiniu_secret_key =
            get_env_value("QINIU_SECRET_KEY").expect("QINIU_SECRET_KEY must be set");
        let qiniu_ai_api_key =
            get_env_value("QINIU_AI_API_KEY").expect("QINIU_AI_API_KEY must be set");
        let mysql_username = get_env_value("MYSQL_USERNAME").expect("MYSQL_USERNAME must be set");
        let mysql_password = get_env_value("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");
        let mysql_endpoint = get_env_value("MYSQL_ENDPOINT").expect("MYSQL_ENDPOINT must be set");
        let qiniu_llm_model = get_env_value_option(
            "QINIU_LLM_MODEL",
            "deepseek/deepseek-v3.1-terminus".to_string(),
        );
        let qiniu_llm_thinking_model =
            get_env_value_option("QINIU_LLM_THINKING_MODEL", "deepseek-r1-0528".to_string());

        Self {
            port,
            tracing_level,
            qiniu_access_key,
            qiniu_secret_key,
            qiniu_ai_api_key,
            mysql_username,
            mysql_password,
            mysql_endpoint,
            qiniu_llm_model,
            qiniu_llm_thinking_model,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use dotenv::dotenv;

    pub fn get_env() -> Env {
        dotenv().ok();

        Env::new()
    }
}
