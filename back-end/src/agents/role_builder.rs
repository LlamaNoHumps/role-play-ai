use super::{AI, remove_prefix_assistant};
use crate::database::models::roles::{Gender, VoiceType};
use anyhow::Result;
use llm_chain::{parameters, prompt};

pub struct RoleBuilt {
    pub description: String,
    pub traits: String,
    pub gender: Gender,
    pub voice_type: VoiceType,
}

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
        gender: &str,
        voice_type: &str,
    ) -> Result<RoleBuilt> {
        let name = self.to_precise_en_title(name).await?;
        let extract = wiki_extract(&name).await?;

        let traits = if extract.is_empty() {
            self.traits_from_prior(&name, description, traits, gender, voice_type)
                .await?
        } else {
            self.traits_from_extract(&extract, description, traits, gender, voice_type)
                .await?
        };

        let prompt = self.build_cn_rp_system_prompt(&name, &traits).await?;

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

        let voice_type = match extract_content_from_xml(&prompt, "voice-type")
            .unwrap_or_default()
            .as_str()
        {
            "mature" => VoiceType::Mature,
            "young" => VoiceType::Young,
            _ => return Err(anyhow::anyhow!("Unknown voice type")),
        };

        Ok(RoleBuilt {
            description,
            traits,
            gender,
            voice_type,
        })
    }

    async fn to_precise_en_title(&self, raw_name: &str) -> Result<String> {
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
        voice_type: &str,
    ) -> Result<String> {
        let sys = r#"
你是角色设定提炼助手。输入是英文维基导言 extract。用户可能会提供一些描述、特征、性别、声音类型。
角色的性别值为“male”或“female”，声音类型为“mature”或“young”。
如果用户提供性别、声音类型，则必须使用这个设定，无论是否与常识冲突，因为这可能是用户的喜好。
如果用户提供描述、特征，则结合extract一并考虑。
请以此用中文输出，提炼适用于“角色扮演”的要点：
- 人物的性格特质（3-6条）
- 说话风格/口吻（2-4条）
- 典型话题或偏好（2-4条）
- 注意事项（禁用话题/需回避的领域，如不了解则写“无明显”）
输出为简洁条目列表，勿包含与事实无关的推断。
必须写明角色的性别和声音类型，且如果用户提供，以用户提供的为准。
"#;

        let res = prompt!(
            sys,
            "英文 extract：\n{{extract}}\n描述：{{description}}\n特征：{{traits}}\n性别：{{gender}}\n声音类型：{{voice_type}}\n——\n请中文列要点。\nAssistant:"
        )
        .run(
            &parameters!("extract" => extract_en, "description" => description, "traits" => traits, "gender" => gender, "voice_type" => voice_type),
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
        voice_type: &str,
    ) -> Result<String> {
        let sys = r#"
你是角色设定助手。当前没有百科 extract。用户可能会提供一些描述、特征、性别、声音类型。
角色的性别值为“male”或“female”，声音类型为“mature”或“young”。
如果用户提供性别、声音类型，则必须使用这个设定，无论是否与常识冲突，因为这可能是用户的喜好。
如果用户提供描述、特征，则结合你所知道的知识一并考虑。
请以此基于你对该人物的常识性认知，生成“可用于角色扮演”的风格要点（中文）。
重要：
- 避免罗列具体生平事实（因无来源），只描述可泛化的“性格/口吻/话题/互动风格”。
- 若不确定，请写“倾向冷静/理性/幽默等通用风格”并给出可用的口癖、语气、回应策略。
- 结构与粒度同：性格特质/说话风格/典型话题/注意事项。
- 必须写明角色的性别和声音类型，且如果用户提供，以用户提供的为准。
"#;

        let res = prompt!(
            sys,
            "人物英文名：{{name}}\n描述：{{description}}\n特征：{{traits}}\n性别：{{gender}}\n声音类型：{{voice_type}}\n请中文列要点。\nAssistant:"
        )
        .run(
            &parameters!("name" => person_en, "description" => description, "traits" => traits, "gender" => gender, "voice_type" => voice_type),
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

    async fn build_cn_rp_system_prompt(&self, person_en: &str, traits_cn: &str) -> Result<String> {
        let sys = r#"
你是提示词工程师。把给定的“角色特征要点”组织成**中文的 system 提示词**，用于和模型对话的角色扮演。
要求：
- 给出角色设定的性别，值为“male”和“female”。用`<gender></gender>`包裹。
- 给出角色设定的声音类型，值为“mature”和“young”。用`<voice-type></voice-type>`包裹。
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

pub fn extract_content_from_xml(xml: &str, tag: &str) -> Option<String> {
    let xml = format!("<root>{}</root>", xml);
    let doc = roxmltree::Document::parse(&xml).ok()?;
    let node = doc.descendants().find(|n| n.has_tag_name(tag))?;
    Some(node.text()?.trim().to_string())
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
        let RoleBuilt {
            description,
            traits,
            gender,
            voice_type,
        } = role_builder
            .build(
                "Harry Potter",
                "朝鲜的工程师",
                "说话很土",
                "female",
                "mature",
            )
            .await
            .unwrap();
        println!(
            "description:\n{}\ntraits:\n{}\ngender:\n{}\nvoice_type:\n{}",
            description, traits, gender, voice_type
        );
    }

    #[tokio::test]
    async fn test_wiki_extract() {
        let title = "Harry Potter";
        let extract = wiki_extract(title).await.unwrap();
        println!("Extract for {}:\n{}", title, extract);
    }
}
