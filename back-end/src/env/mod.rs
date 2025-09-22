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
    pub firecrawl_api_key: String,
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
        let firecrawl_api_key =
            get_env_value("FIRECRAWL_API_KEY").expect("FIRECRAWL_API_KEY must be set");

        Self {
            port,
            tracing_level,
            qiniu_access_key,
            qiniu_secret_key,
            qiniu_ai_api_key,
            firecrawl_api_key,
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
