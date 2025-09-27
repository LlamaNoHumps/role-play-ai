use super::AI;
use crate::{agents::remove_prefix_assistant, database::Database};
use anyhow::Result;
use llm_chain::{parameters, prompt};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Summarizer {
    ai: AI,
    database: Arc<Database>,
    tx: mpsc::UnboundedSender<SummarizerTask>,
}

impl Summarizer {
    pub fn new(ai: AI, database: Arc<Database>) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let service = Self { ai, database, tx };

        let service_clone = service.clone();
        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                if let Err(e) = service_clone.process_task(task).await {
                    tracing::error!("Summarizer task failed: {}", e);
                }
            }
        });

        service
    }

    async fn summarize(&self, context: &str) -> Result<String> {
        let sys = r#"
你是一个对话记忆提取器。请根据以下对话内容，提炼出关键的长期信息，供后续对话参考。
要求：
1) 只提炼对后续对话有帮助的长期信息，忽略无关内容。
2) 指明人物，按人物分段输出。
3) 若无明显长期信息，输出“无”。
"#;

        let res = prompt!(sys, "对话内容：{{context}}\n请提炼长期信息：")
            .run(
                &parameters!("context" => context),
                &self.ai.thinking_executor,
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

    async fn merge(&self, old: &str, new: &str) -> Result<String> {
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
        .run(
            &parameters!("old" => old, "new" => new),
            &self.ai.thinking_executor,
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

    pub async fn check_and_trigger(&self, user_id: i32, role_id: i32) -> Result<()> {
        let count = self.database.get_dialog_count(user_id, role_id).await?;

        if count > 0 && count % 10 == 0 {
            self.tx.send(SummarizerTask { user_id, role_id })?;
        }

        Ok(())
    }

    async fn process_task(&self, task: SummarizerTask) -> Result<()> {
        tracing::info!(
            "Processing summarizer task for user {} role {}",
            task.user_id,
            task.role_id
        );

        let recent_dialogs = self
            .database
            .get_recent_dialogs(task.user_id, task.role_id, 10)
            .await?;
        if recent_dialogs.is_empty() {
            return Ok(());
        }

        let recent_context = recent_dialogs
            .iter()
            .map(|d| {
                if d.is_user {
                    format!("User: {}", d.text)
                } else {
                    format!("Assistant: {}", d.text)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        let new_summary = self.summarize(&recent_context).await?;

        if new_summary.trim() == "无" {
            return Ok(());
        }

        let conversation = self
            .database
            .get_conversation(task.user_id, task.role_id)
            .await?;
        let old_history = conversation
            .map(|c| c.history)
            .filter(|h| !h.is_empty() && h != "无")
            .unwrap_or_default();

        let final_history = if old_history.is_empty() {
            new_summary
        } else {
            self.merge(&old_history, &new_summary).await?
        };

        self.database
            .update_conversation_history(task.user_id, task.role_id, &final_history)
            .await?;

        tracing::info!(
            "Summarizer task completed for user {} role {}",
            task.user_id,
            task.role_id
        );
        Ok(())
    }
}

pub struct SummarizerTask {
    pub user_id: i32,
    pub role_id: i32,
}
