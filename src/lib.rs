use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifyResponse {
    pub sequence: String,
    pub labels: Vec<String>,
    pub scores: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub compute_time: f32,
    pub text: String,
    pub prompt: String,
}

pub struct GPT {
    pub url: String,
}

impl GPT {
    pub async fn classify(
        &self,
        labels: &Vec<String>,
        sequence: String,
    ) -> Result<ClassifyResponse, Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/classify", self.url))
            .query(&[("labels", labels.join(",")), ("sequence", sequence)])
            .send()
            .await?;
        let json: ClassifyResponse = response.json::<ClassifyResponse>().await?;
        Ok(json)
    }

    pub async fn generate(
        &self,
        context: String,
        token_max_length: u16,
        temperature: f32,
        top_p: f32,
        stop_sequence: Option<String>,
    ) -> Result<GenerateResponse, Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/generate", self.url))
            .query(&[
                ("context", context),
                ("token_max_length", token_max_length.to_string()),
                ("temperature", temperature.to_string()),
                ("top_p", top_p.to_string()),
                ("stop_sequence", stop_sequence.unwrap_or("\"\"".to_string())),
            ])
            .send()
            .await?;
        let json: GenerateResponse = response.json::<GenerateResponse>().await?;
        Ok(json)
    }
}

impl Default for GPT {
    fn default() -> Self {
        GPT {
            url: "http://api.vicgalle.net:5000".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::GPT;

    #[tokio::test]
    async fn test_classify() {
        let gpt: GPT = GPT::default();
        let labels: Vec<String> = vec![
            "positive".to_string(),
            "neutral".to_string(),
            "negative".to_string(),
        ];
        let sequence: String = "In a shocking finding, scientists discovered a herd of unicorns living in a remote, previously unexplored valley, in the Andes Mountains. Even more surprising to the researchers was the fact that the unicorns spoke perfect English.".to_string();
        let response = gpt.classify(&labels, sequence).await.unwrap();
        println!("{:?}", response);
        assert_eq!(response.labels[0], "positive");
    }

    #[tokio::test]
    async fn test_generate() {
        let gpt: GPT = GPT::default();
        let context = "Hello World in C++".to_string();
        let response = gpt.generate(context, 255, 0.9, 0.9, None).await.unwrap();
        println!("{:?}", response);
    }
}
