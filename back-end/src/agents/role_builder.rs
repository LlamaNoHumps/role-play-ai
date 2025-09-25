use std::sync::Arc;

use super::AI;
use anyhow::Result;
use axum::Extension;
use llm_chain::{parameters, prompt};

pub struct RoleBuilder {
    ai: AI,
}

impl RoleBuilder {
    pub fn new(ai: AI) -> Self {
        Self { ai }
    }

    pub async fn build(
        &self,
        name: &str,
        description: &str,
        traits: &str,
    ) -> Result<(String, String)> {
        let name = self.to_precise_en_title(name).await?;
        println!("Precise English title: {}", name);
        let extract = wiki_extract(&name).await?;
        println!("Wiki extract: {}", extract);

        let traits = if extract.is_empty() {
            self.traits_from_prior(&name, description, traits).await?
        } else {
            self.traits_from_extract(&extract, description, traits)
                .await?
        };

        println!("Extracted traits:\n{}", traits);

        let prompt = self.build_cn_rp_system_prompt(&name, &traits).await?;
        let prompt = prompt.trim_start_matches("Assistant:").trim().to_string();

        println!("Final prompt:\n{}", prompt);
        let parts: Vec<&str> = prompt.split("<br>").collect();
        println!("Prompt parts: {:?}", parts);

        if parts.len() == 2 {
            let description = parts[0].trim().to_string();
            let traits = parts[1].trim().to_string();
            Ok((description, traits))
        } else {
            Err(anyhow::anyhow!(
                "Prompt format error: expected 2 parts separated by <br>"
            ))
        }
    }

    async fn to_precise_en_title(&self, raw_name: &str) -> Result<String> {
        let sys = r#"
你是维基百科检索助手。目标：把用户的人名转成“维基百科英文条目标题”。
要求：
1) 返回单行纯文本，不要解释、不要引号。
2) 如遇多义/同名，返回最常见且与“人物”对应的英文标题（不含括号 disambiguation）。
3) 若用户已是英文名，规范大小写与常见写法（如 'Jay Chou'）。
"#;

        let res = prompt!(sys, "用户给出的人名：{{name}}\n仅输出英文标题：")
            .run(&parameters!("name" => raw_name), &self.ai.executor)
            .await?;

        Ok(res
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string())
    }

    async fn traits_from_extract(
        &self,
        extract_en: &str,
        description: &str,
        traits: &str,
    ) -> Result<String> {
        let sys = r#"
你是角色设定提炼助手。输入是英文维基导言 extract。用户可能会提供一些描述和特征。
请用中文输出，提炼适用于“角色扮演”的要点：
- 人物的性格特质（3-6条）
- 说话风格/口吻（2-4条）
- 典型话题或偏好（2-4条）
- 注意事项（禁用话题/需回避的领域，如不了解则写“无明显”）
输出为简洁条目列表，勿包含与事实无关的推断。
"#;

        let res = prompt!(
            sys,
            "英文 extract：\n{{extract}}\n描述：{{description}}\n特征：{{traits}}——\n请中文列要点："
        )
        .run(
            &parameters!("extract" => extract_en, "description" => description, "traits" => traits),
            &self.ai.executor,
        )
        .await?;
        Ok(res
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string())
    }

    async fn traits_from_prior(
        &self,
        person_en: &str,
        description: &str,
        traits: &str,
    ) -> Result<String> {
        let sys = r#"
你是角色设定助手。当前没有百科 extract。用户可能会提供一些描述和特征。
请基于你对该人物的常识性认知，生成“可用于角色扮演”的风格要点（中文）。
重要：
- 避免罗列具体生平事实（因无来源），只描述可泛化的“性格/口吻/话题/互动风格”。
- 若不确定，请写“倾向冷静/理性/幽默等通用风格”并给出可用的口癖、语气、回应策略。
- 结构与粒度同：性格特质/说话风格/典型话题/注意事项。
"#;

        let res = prompt!(
            sys,
            "人物英文名：{{name}}\n描述：{{description}}\n特征：{{traits}}\n请中文列要点："
        )
        .run(
            &parameters!("name" => person_en, "description" => description, "traits" => traits),
            &self.ai.executor,
        )
        .await?;
        Ok(res
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string())
    }

    async fn build_cn_rp_system_prompt(&self, person_en: &str, traits_cn: &str) -> Result<String> {
        let sys = r#"
你是提示词工程师。把给定的“角色特征要点”组织成**中文的 system 提示词**，用于和模型对话的角色扮演。
要求：
- 开头一段用中文概述人物设定（含英文名作参照）。
- 列出“说话风格/互动规则/可谈话题/回避事项”。
- 给出 3 条示例口癖或措辞模板（中文）。
- 控制在 250~400 字，适合作为 system。
- 只输出提示词部分。
- 人物设定这一段需要与后面的其余部分用<br>分开。
"#;

        let res = prompt!(
            sys,
            "人物（英文名）：{{name}}\n特征要点：\n{{traits}}\n——\n请生成中文 system："
        )
        .run(
            &parameters!("name" => person_en, "traits" => traits_cn),
            &self.ai.executor,
        )
        .await?;
        Ok(res
            .to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string())
    }

    pub fn into_layer(self) -> Extension<Arc<Self>> {
        Extension(Arc::new(self))
    }
}

pub async fn wiki_extract(title: &str) -> Result<String> {
    let api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php").await?;

    let params = api.params_into(&[
        ("action", "query"),
        ("prop", "extracts"),
        ("exintro", ""),
        ("explaintext", ""),
        ("titles", title),
        ("format", "json"),
    ]);

    let res = api.get_query_api_json_all(&params).await?;

    let pages = &res["query"]["pages"];

    // page编号为-1则没有找到，没有extract
    if pages.as_object().unwrap().contains_key("-1") {
        return Ok(String::new());
    }

    // 取第一个page
    let page = pages.as_object().unwrap().values().next().unwrap();
    let extract = page["extract"].as_str().unwrap_or("").to_string();

    Ok(extract)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_role_builder() {
        let env = get_env();
        let ai = AI::new(&env.qiniu_ai_api_key);

        let role_builder = RoleBuilder::new(ai);
        let (description, traits) = role_builder.build("Albert Einstein", "", "").await.unwrap();
        println!("description:\n{}\n{}", description, traits);
    }

    #[tokio::test]
    async fn test_wiki_extract() {
        let title = "Albert Einstein";
        let extract = wiki_extract(title).await.unwrap();
        println!("Extract for {}:\n{}", title, extract);
    }
}
