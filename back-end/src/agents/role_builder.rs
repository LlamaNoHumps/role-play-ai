use super::{AI, remove_prefix_assistant};
use crate::{
    agents::Reciter,
    database::models::roles::{AgeGroup, Gender},
};
use anyhow::{Result, anyhow};
use llm_chain::{parameters, prompt};
use socketioxide::SocketIo;

pub struct RoleBuilt {
    pub description: String,
    pub traits: String,
    pub gender: Gender,
    pub age_group: AgeGroup,
    pub voice_type: String,
}

pub struct RoleBuilder {
    ai: AI,
    socket: Option<SocketIo>,
    reciter: Reciter,
}

impl RoleBuilder {
    pub fn new(ai: AI, socket: Option<SocketIo>, reciter: Reciter) -> Self {
        Self {
            ai,
            socket,
            reciter,
        }
    }

    pub async fn build(
        &self,
        name: &str,
        description: &str,
        traits: &str,
        gender: &str,
        age_group: &str,
        sid: Option<String>,
    ) -> Result<RoleBuilt> {
        let name = self.to_precise_en_title(name, sid.clone()).await?;
        let extract = self.wiki_extract(&name, sid.clone()).await?;

        let traits = if extract.is_empty() {
            self.traits_from_prior(&name, description, traits, gender, age_group, sid.clone())
                .await?
        } else {
            self.traits_from_extract(
                &extract,
                description,
                traits,
                gender,
                age_group,
                sid.clone(),
            )
            .await?
        };

        let voice_type = self.select_voice_type(&traits, sid.clone()).await?;

        let prompt = self.build_cn_rp_system_prompt(&name, &traits, sid).await?;

        let description = extract_content_from_xml(&prompt, "description").unwrap_or_default();

        let traits = extract_content_from_xml(&prompt, "traits").unwrap_or_default();

        let gender = match extract_content_from_xml(&prompt, "gender")
            .unwrap_or_default()
            .as_str()
        {
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => return Err(anyhow::anyhow!("Unknown gender")),
        };

        let age_group = match extract_content_from_xml(&prompt, "age-group")
            .unwrap_or_default()
            .as_str()
        {
            "mature" => AgeGroup::Mature,
            "young" => AgeGroup::Young,
            _ => return Err(anyhow::anyhow!("Unknown age group")),
        };

        Ok(RoleBuilt {
            description,
            traits,
            gender,
            age_group,
            voice_type,
        })
    }

    async fn to_precise_en_title(&self, raw_name: &str, sid: Option<String>) -> Result<String> {
        self.emit_status(sid.clone(), "正在规范角色名称...").await?;

        let sys = r#"
你是维基百科检索助手。目标：把用户的人名转成“维基百科英文条目标题”。
要求：
- 返回单行纯文本，不要解释、不要引号。把人名包裹在`<name></name>`标签中。
- 如遇多义/同名，返回最常见且与“人物”对应的英文标题（不含括号 disambiguation）。
- 若用户已是英文名，规范大小写与常见写法（如 'Jay Chou'）。
- 你的回复必须是xml格式，且只能包含上述四个标签，且每个标签只能出现一次，且标签内不能嵌套其他标签。
"#;

        let res = prompt!(
            sys,
            "用户给出的人名：{{name}}\n仅输出英文标题。\nAssistant:"
        )
        .run(&parameters!("name" => raw_name), &self.ai.executor)
        .await?
        .to_immediate()
        .await?
        .as_content()
        .to_text()
        .trim()
        .to_string();

        let res = remove_prefix_assistant(&res);

        let name = extract_content_from_xml(res, "name").ok_or(anyhow::anyhow!(
            "Failed to extract name from response: {}",
            res
        ))?;

        Ok(name)
    }

    async fn traits_from_extract(
        &self,
        extract_en: &str,
        description: &str,
        traits: &str,
        gender: &str,
        age_group: &str,
        sid: Option<String>,
    ) -> Result<String> {
        self.emit_status(sid, "正在生成角色特征...").await?;

        let sys = r#"
你是角色设定提炼助手。输入是英文维基导言 extract。用户可能会提供一些描述、特征、性别、年龄段。
角色的性别值为“male”或“female”，年龄段为“mature”或“young”。
如果用户提供性别、年龄段，则必须使用这个设定，无论是否与常识冲突，因为这可能是用户的喜好。
如果用户提供描述、特征，则结合extract一并考虑。
请以此用中文输出，提炼适用于“角色扮演”的要点：
- 人物的性格特质（3-6条）
- 说话风格/口吻（2-4条）
- 典型话题或偏好（2-4条）
- 注意事项（禁用话题/需回避的领域，如不了解则写“无明显”）
输出为简洁条目列表，勿包含与事实无关的推断。
必须写明角色的性别和年龄段，且如果用户提供，以用户提供的为准。
"#;

        let res = prompt!(
            sys,
            "英文 extract：\n{{extract}}\n描述：{{description}}\n特征：{{traits}}\n性别：{{gender}}\n年龄段：{{age_group}}\n——\n请中文列要点。\nAssistant:"
        )
        .run(
            &parameters!("extract" => extract_en, "description" => description, "traits" => traits, "gender" => gender, "age_group" => age_group),
            &self.ai.executor,
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

    async fn traits_from_prior(
        &self,
        person_en: &str,
        description: &str,
        traits: &str,
        gender: &str,
        age_group: &str,
        sid: Option<String>,
    ) -> Result<String> {
        self.emit_status(sid, "正在生成角色特征...").await?;

        let sys = r#"
你是角色设定助手。当前没有百科 extract。用户可能会提供一些描述、特征、性别、年龄段。
角色的性别值为“male”或“female”，年龄段为“mature”或“young”。
如果用户提供性别、年龄段，则必须使用这个设定，无论是否与常识冲突，因为这可能是用户的喜好。
如果用户提供描述、特征，则结合你所知道的知识一并考虑。
请以此基于你对该人物的常识性认知，生成“可用于角色扮演”的风格要点（中文）。
重要：
- 避免罗列具体生平事实（因无来源），只描述可泛化的“性格/口吻/话题/互动风格”。
- 若不确定，请写“倾向冷静/理性/幽默等通用风格”并给出可用的口癖、语气、回应策略。
- 结构与粒度同：性格特质/说话风格/典型话题/注意事项。
- 必须写明角色的性别和年龄段，且如果用户提供，以用户提供的为准。
"#;

        let res = prompt!(
            sys,
            "人物英文名：{{name}}\n描述：{{description}}\n特征：{{traits}}\n性别：{{gender}}\n年龄段：{{age_group}}\n请中文列要点。\nAssistant:"
        )
        .run(
            &parameters!("name" => person_en, "description" => description, "traits" => traits, "gender" => gender, "age_group" => age_group),
            &self.ai.executor,
        )
        .await?.to_immediate()
            .await?
            .as_content()
            .to_text()
            .trim()
            .to_string();

        let res = remove_prefix_assistant(&res);

        Ok(res.to_string())
    }

    async fn select_voice_type(&self, traits: &str, sid: Option<String>) -> Result<String> {
        self.emit_status(sid, "正在选择角色声音类型...").await?;

        let voice_map = self.reciter.fetch_voice_map().await?;
        let voice_map_json = serde_json::to_string_pretty(&voice_map)?;

        let sys = format!(
            r#"
你是角色声音选择助手。根据给出的角色特征，选择合适的声音类型：
可用声音json：{}
要求：
- 仅输出选取的声音的键名（字符串），不要解释。
- 请通过角色的性格、年龄、说话风格等综合判断。
"#,
            voice_map_json
        );

        let res = prompt!(
            &sys,
            "角色特征：\n{{traits}}\n——\n请输出声音类型键名。\nAssistant:"
        )
        .run(&parameters!("traits" => traits), &self.ai.executor)
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

    async fn build_cn_rp_system_prompt(
        &self,
        person_en: &str,
        traits_cn: &str,
        sid: Option<String>,
    ) -> Result<String> {
        self.emit_status(sid, "正在生成角色描述和特点...").await?;

        let sys = r#"
你是提示词工程师。把给定的“角色特征要点”组织成**中文的 system 提示词**，用于和模型对话的角色扮演。
要求：
- 给出角色设定的性别，值为“male”和“female”。用`<gender></gender>`包裹。
- 给出角色设定的年龄段，值为“mature”和“young”。用`<age-group></age-group>`包裹。
- 用中文概述人物设定，附上英文对照，需要包含人物性别和年纪大小。用`<description></description>`包裹。
- 列出“说话风格/互动规则/可谈话题/回避事项”，并给出 3 条示例口癖或措辞模板（中文）。用`<traits></traits>`包裹。
- 控制在 250~400 字，适合作为 system。
- 你的回复必须是xml格式，且只能包含上述四个标签，且每个标签只能出现一次，且标签内不能嵌套其他标签。
"#;

        let res = prompt!(
            sys,
            "人物（英文名）：{{name}}\n特征要点：\n{{traits}}\n——\n请生成中文\nAssistant:"
        )
        .run(
            &parameters!("name" => person_en, "traits" => traits_cn),
            &self.ai.executor,
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

    pub async fn wiki_extract(&self, title: &str, sid: Option<String>) -> Result<String> {
        self.emit_status(sid, "正在尝试从维基百科获取角色简介...")
            .await?;

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

    async fn emit_status(&self, sid: Option<String>, status: &str) -> Result<()> {
        if let Some(socket) = &self.socket {
            if let Some(sid) = sid {
                socket
                    .to(sid)
                    .emit("role_build_status", status)
                    .await
                    .map_err(|e| anyhow!("{}", e))?;
            }
        }

        Ok(())
    }
}

pub fn extract_content_from_xml(xml: &str, tag: &str) -> Option<String> {
    let xml = format!("<root>{}</root>", xml);
    let doc = roxmltree::Document::parse(&xml).ok()?;
    let node = doc.descendants().find(|n| n.has_tag_name(tag))?;
    Some(node.text()?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{env::tests::get_env, storage::StorageClient};

    #[tokio::test]
    async fn test_role_builder() {
        let env = get_env();
        let ai = AI::new(
            &env.qiniu_ai_api_key,
            &env.qiniu_llm_model,
            &env.qiniu_llm_thinking_model,
        );
        let storage_client = Arc::new(StorageClient::new(
            &env.qiniu_access_key,
            &env.qiniu_secret_key,
        ));
        let reciter = Reciter::new(storage_client, &env.qiniu_ai_api_key);

        let role_builder = RoleBuilder::new(ai, None, reciter);
        let RoleBuilt {
            description,
            traits,
            gender,
            age_group,
            voice_type,
        } = role_builder
            .build("爱因斯坦", "", "", "", "", None)
            .await
            .unwrap();
        println!(
            "description:\n{}\ntraits:\n{}\ngender:\n{}\nage_group:\n{}\nvoice_type:\n{}",
            description, traits, gender, age_group, voice_type
        );
    }

    #[tokio::test]
    async fn test_wiki_extract() {
        let env = get_env();
        let ai = AI::new(
            &env.qiniu_ai_api_key,
            &env.qiniu_llm_model,
            &env.qiniu_llm_thinking_model,
        );
        let storage_client = Arc::new(StorageClient::new(
            &env.qiniu_access_key,
            &env.qiniu_secret_key,
        ));
        let reciter = Reciter::new(storage_client, &env.qiniu_ai_api_key);

        let role_builder = RoleBuilder::new(ai, None, reciter);

        let title = "Harry Potter";
        let extract = role_builder.wiki_extract(title, None).await.unwrap();
        println!("Extract for {}:\n{}", title, extract);
    }
}
