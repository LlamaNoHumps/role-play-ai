use super::AI;
use crate::database::Database;
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct Debater {
    ai: AI,
    database: Arc<Database>,
}

impl Debater {
    pub fn new(ai: AI, database: Arc<Database>) -> Self {
        Self { ai, database }
    }

    pub async fn answer(
        &self,
        user_id: i32,
        role1_id: i32,
        role2_id: i32,
        role_id: i32,
        is_starting: bool,
    ) -> Result<String> {
        let role1 = self.database.get_role(role1_id).await?;
        let role2 = self.database.get_role(role2_id).await?;
        let topic = self
            .database
            .get_debate(user_id, role1_id, role2_id)
            .await?
            .unwrap()
            .topic;

        let (current_role, other_role) = if role_id == role1_id {
            (role1, role2)
        } else {
            (role2, role1)
        };

        let system = if is_starting {
            format!(
                "{}\n\n现在你正在进行一场辩论，主题是{}。这是对方的角色信息：\n{}\n{}\n\n作为辩论的开始者，请开始你的论述。请保持角色设定，展现你的观点和立场。发言应该简洁有力。",
                current_role.prompt(),
                topic,
                other_role.description,
                other_role.traits
            )
        } else {
            format!(
                "{}\n\n现在你正在进行一场辩论，主题是{}。这是对方的角色信息：\n{}\n{}\n\n根据之前的对话历史，请继续你的辩论发言。要针对对方的观点进行回应，并提出你的新论点。请保持角色设定，发言应该简洁有力。",
                current_role.prompt(),
                topic,
                other_role.description,
                other_role.traits
            )
        };

        let user = if is_starting {
            format!("请开始关于《{}》的辩论", topic)
        } else {
            "请根据对话历史继续辩论".to_string()
        };

        let history = self
            .database
            .get_debate_history(user_id, role1_id, role2_id)
            .await?;

        let answer = self.ai.chat_once(&system, &user, Some(&history)).await?;

        Ok(answer)
    }
}
