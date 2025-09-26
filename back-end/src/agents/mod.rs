pub mod role_builder;
mod summarizer;

use anyhow::Result;
use async_openai::config::OpenAIConfig;
use llm_chain::{
    options::{ModelRef, Opt, Options},
    parameters, prompt,
};
use llm_chain_openai::chatgpt::Executor;

pub use role_builder::RoleBuilder;
pub use summarizer::Summarizer;

#[derive(Clone)]
pub struct AI {
    executor: Executor,
    thinking_executor: Executor,
}

impl AI {
    pub fn new(api_key: &str) -> Self {
        let config = OpenAIConfig::new()
            .with_api_base("https://openai.qiniu.com/v1")
            .with_api_key(api_key);
        let client = async_openai::Client::with_config(config);

        let mut options_builder = Options::builder();
        options_builder.add_option(Opt::ApiKey(api_key.to_string()));
        options_builder.add_option(Opt::Model(ModelRef::from_model_name(
            "deepseek/deepseek-v3.1-terminus",
        )));
        options_builder.add_option(Opt::Stream(false));

        let options = options_builder.build();

        let executor = Executor::for_client(client.clone(), options);

        let mut options_builder = Options::builder();
        options_builder.add_option(Opt::ApiKey(api_key.to_string()));
        options_builder.add_option(Opt::Model(ModelRef::from_model_name("deepseek-r1-0528")));
        options_builder.add_option(Opt::Stream(false));

        let options = options_builder.build();

        let thinking_executor = Executor::for_client(client, options);

        Self {
            executor,
            thinking_executor,
        }
    }

    pub async fn chat_once(
        &self,
        system: &str,
        user: &str,
        history: Option<&str>,
    ) -> Result<String> {
        let history = history.unwrap_or_default();

        println!("System Prompt:\n{}", system);
        println!("History:\n{}", history);
        println!("User Input:\n{}", user);

        let res = prompt!("{{system}}\n{{history}}", "{{user}}\nAssistant:")
            .run(
                &parameters!("system" => system, "history" => history, "user" => user),
                &self.thinking_executor,
            )
            .await?
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string();

        let res = remove_prefix_assistant(&res);

        Ok(res.to_string())
    }
}

pub fn remove_prefix_assistant(text: &str) -> &str {
    text.trim_start_matches("Assistant:").trim()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_chat() {
        let prompt = r#"Assistant: 
### Albert Einstein（阿尔伯特·爱因斯坦）角色设定  
你扮演爱因斯坦，一位睿智深邃的物理学家（Albert Einstein），充满孩童般的好奇心，谦逊幽默，关注人类福祉与理想主义，常因沉思显得心不在焉，拒绝权威说教。

**说话风格**  
- 善用生活化比喻解释抽象理论（如时间相对性）  
- 语调平静诗意，穿插哲学反问（如“上帝掷骰子吗？”）  
- 口癖：思考时沉吟“嗯……”，强调观点前用“你看……”邀请探讨  
- 幽默策略：以荒诞假设解构问题（如“如果我追光……”）  

**互动规则**  
- 保持谦逊，注重思考过程而非结论  
- 邀请对话而非教导，允许停顿模拟构建思维  
- 流露对未知的敬畏，弱化天才标签  

**可谈话题**  
- 宇宙奥秘：时间本质、光速悖论、统一场论的哲学意义  
- 人类困境：教育批判（如想象力优先）、反战思想、宗教与科学关系  
- 艺术与科学共生：小提琴灵感、科学中的美学  
- 思想实验：探讨“如果……会怎样”的开放命题  

**回避事项**  
- 避免数学细节或公式推导  
- 警惕刻板印象如疯癫发型，慵懒源于沉思而非邋遢  
- 拒绝教导姿态，互动留白允许自然停顿  

**示例口癖/措辞**  
1. “嗯……让我想想……”（虚拟捻胡子）  
2. “你看，这就像……”（引入比喻）  
3. “假如我们以光速飞行，会发生什么？”（荒诞假设）"#;

        let env = get_env();
        let ai = AI::new(&env.qiniu_ai_api_key);
        let reply = ai
            .chat_once(prompt, "你最近在忙什么呀？", None)
            .await
            .unwrap();
        println!("Reply:\n{}", reply);
    }
}
