use anyhow::Result;
use firecrawl::{
    FirecrawlApp,
    // scrape::{ScrapeFormats, ScrapeOptions},
    search::SearchParams,
};

pub struct Crawler {
    client: FirecrawlApp,
}

impl Crawler {
    pub fn new(api_key: &str) -> Self {
        let client = FirecrawlApp::new(api_key).unwrap();

        Self { client }
    }

    pub async fn search(&self, content: &str) -> Result<String> {
        let query = format!("{} 的 性格 事迹", content);
        let search_params = SearchParams {
            query: query.clone(),
            limit: Some(20),
            timeout: Some(3000),
            // scrape_options: Some(ScrapeOptions {
            //     formats: Some(vec![ScrapeFormats::Markdown]),
            //     ..Default::default()
            // }),
            ..Default::default()
        };

        let res = self.client.search(query, Some(search_params)).await?;

        let result = res
            .data
            .iter()
            .map(|item| item.description.clone())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_search() {
        let env = get_env();

        let crawler = Crawler::new(&env.firecrawl_api_key);
        let result = crawler.search("哈利波特").await;
        println!("{:?}", result);
    }
}
