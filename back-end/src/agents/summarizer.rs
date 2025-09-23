use super::AI;
use anyhow::Result;
use llm_chain::{parameters, prompt};

pub struct Summarizer {
    ai: AI,
}

impl Summarizer {
    pub fn new(ai: AI) -> Self {
        Self { ai }
    }

    pub async fn summarize(&self, context: &str) -> Result<String> {
        let sys = r#"
你是一个对话记忆提取器。请根据以下对话内容，提炼出关键的长期信息，供后续对话参考。
要求：
1) 只提炼对后续对话有帮助的长期信息，忽略无关内容。
2) 指明人物，按人物分段输出。
3) 若无明显长期信息，输出“无”。
"#;

        let res = prompt!(sys, "对话内容：{{context}}\n请提炼长期信息：")
            .run(&parameters!("context" => context), &self.ai.executor)
            .await?;
        Ok(res
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string())
    }

    pub async fn merge(&self, old: &str, new: &str) -> Result<String> {
        let sys = r#"
你是一个对话记忆合并助手。请将已有的长期信息与新提炼的长期信息合并，形成更完整的长期信息。
要求：
1) 合并时去重，确保信息简洁。
2) 若新信息与已有信息冲突，保留新信息。
3) 指明人物，按人物分段输出。
4) 若两者均为“无”，输出“无”。
"#;

        let res = prompt!(
            sys,
            "已有长期信息：{{old}}\n新提炼的长期信息：{{new}}\n请合并为更完整的长期信息："
        )
        .run(&parameters!("old" => old, "new" => new), &self.ai.executor)
        .await?;
        Ok(res
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string())
    }
}
