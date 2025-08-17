use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::time::Duration;
use crate::content::ContentType;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: i32,
    top_p: f32,
    top_k: i32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minutes for local models
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;
        
        Ok(Self {
            client,
            base_url,
        })
    }
    
    pub async fn generate_text(&self, model: &str, prompt: &str, max_tokens: i32, temperature: f32) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature,
                num_predict: max_tokens,
                top_p: 0.9,
                top_k: 40,
            },
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Request failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            
            let error_msg = if status == 404 {
                if error_text.contains("model") || error_text.contains("not found") {
                    format!("Model '{}' not found in Ollama. Run 'ollama pull {}' to download it first.", model, model)
                } else {
                    format!("Ollama server not found. Make sure Ollama is running on {}. Install from https://ollama.ai", self.base_url)
                }
            } else {
                format!("Ollama request failed with status {}: {}", status, error_text)
            };
            
            return Err(anyhow!(error_msg));
        }
        
        let ollama_response: OllamaResponse = response.json().await
            .map_err(|e| anyhow!("Failed to parse Ollama response: {}", e))?;
        
        Ok(ollama_response.response)
    }
    
    pub async fn check_server(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
    
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get model list: {}", e))?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get model list from Ollama"));
        }
        
        let response_json: serde_json::Value = response.json().await
            .map_err(|e| anyhow!("Failed to parse model list: {}", e))?;
        
        let mut models = Vec::new();
        if let Some(model_array) = response_json.get("models").and_then(|m| m.as_array()) {
            for model in model_array {
                if let Some(name) = model.get("name").and_then(|n| n.as_str()) {
                    models.push(name.to_string());
                }
            }
        }
        
        Ok(models)
    }
    
    pub async fn generate_chapter(&self, model: &str, context: &str, chapter_outline: &str, target_words: usize) -> Result<String> {
        let max_tokens = (target_words as f32 * 1.3) as i32; // Approximate token count
        
        let prompt = format!(
            "Context: {}\n\nChapter Outline: {}\n\nWrite a detailed chapter based on the outline above. Target length: {} words.\n\nChapter:",
            context, chapter_outline, target_words
        );
        
        self.generate_text(model, &prompt, max_tokens, 0.8).await
    }
    
    pub async fn generate_outline(&self, model: &str, genre: &str, style: &str, premise: &str, num_chapters: usize) -> Result<String> {
        let prompt = format!(
            "Create a detailed outline for a {} book in {} style.\n\
            Premise: {}\n\
            Create exactly {} chapters with brief descriptions.\n\
            Format as:\n\
            Chapter 1: [Title] - [Brief description]\n\
            Chapter 2: [Title] - [Brief description]\n\
            ...\n\n\
            Outline:",
            genre, style, premise, num_chapters
        );
        
        self.generate_text(model, &prompt, 1000, 0.7).await
    }
    
    // New methods for different content types
    pub async fn generate_content_outline(&self, model: &str, content_type: &ContentType, genre: &str, style: &str, premise: &str, num_sections: usize) -> Result<String> {
        let (content_name, section_name) = match content_type {
            ContentType::Book => ("book", "chapter"),
            ContentType::Screenplay => ("screenplay", "scene"),
            ContentType::Play => ("stage play", "act"),
            ContentType::TvScript => ("TV script", "episode"),
            ContentType::AudioScript => ("audio script", "segment"),
            ContentType::GameScript => ("game script", "interaction"),
            ContentType::Document => ("document", "section"),
        };
        
        let specific_instructions = match content_type {
            ContentType::Screenplay => "Include scene headings like 'INT. LOCATION - DAY' or 'EXT. LOCATION - NIGHT'.",
            ContentType::Play => "Include act divisions and stage directions.",
            ContentType::TvScript => "Include episode structure with act breaks.",
            ContentType::AudioScript => "Include audio cues and timing considerations.",
            ContentType::GameScript => "Include character interactions and dialogue choices.",
            ContentType::Document => "Include logical section flow and key topics.",
            ContentType::Book => "Include character development and plot progression.",
        };
        
        let prompt = format!(
            "Create a detailed outline for a {} {} in {} style.\n\
            Premise: {}\n\
            Create exactly {} {}s with brief descriptions.\n\
            {}\n\
            Format as:\n\
            {} 1: [Title] - [Brief description]\n\
            {} 2: [Title] - [Brief description]\n\
            ...\n\n\
            Outline:",
            genre, content_name, style, premise, num_sections, section_name,
            specific_instructions,
            section_name.to_uppercase(), section_name.to_uppercase()
        );
        
        self.generate_text(model, &prompt, 1000, 0.7).await
    }
    
    pub async fn generate_content_section(&self, model: &str, content_type: &ContentType, context: &str, section_outline: &str, target_words: usize) -> Result<String> {
        let max_tokens = (target_words as f32 * 1.3) as i32;
        
        let formatting_instructions = match content_type {
            ContentType::Screenplay => {
                "\n\nFormat as a screenplay with:\n\
                - Scene headings in CAPS (INT./EXT. LOCATION - TIME)\n\
                - Character names in CAPS before dialogue\n\
                - Action lines in present tense\n\
                - Parentheticals for actor direction"
            },
            ContentType::Play => {
                "\n\nFormat as a stage play with:\n\
                - Character names in CAPS followed by colon\n\
                - Stage directions in [brackets]\n\
                - Clear dialogue formatting"
            },
            ContentType::TvScript => {
                "\n\nFormat as a TV script with:\n\
                - Scene transitions\n\
                - Character names in CAPS\n\
                - Commercial break considerations"
            },
            ContentType::AudioScript => {
                "\n\nFormat as an audio script with:\n\
                - NARRATOR: for narration\n\
                - SFX: for sound effects\n\
                - MUSIC: for music cues\n\
                - Character dialogue clearly marked"
            },
            ContentType::GameScript => {
                "\n\nFormat as a game script with:\n\
                - CHOICE: for player dialogue options\n\
                - ACTION: for game actions\n\
                - CONDITION: for branching logic\n\
                - Character dialogue with emotion tags"
            },
            ContentType::Document => {
                "\n\nFormat as a professional document with:\n\
                - Clear headings and subheadings\n\
                - Bullet points where appropriate\n\
                - Professional tone and structure"
            },
            ContentType::Book => {
                "\n\nFormat as a book chapter with:\n\
                - Descriptive prose\n\
                - Character development\n\
                - Scene setting and atmosphere"
            },
        };
        
        let prompt = format!(
            "Context: {}\n\nSection Outline: {}\n\nWrite detailed content based on the outline above. Target length: {} words.{}\n\nContent:",
            context, section_outline, target_words, formatting_instructions
        );
        
        self.generate_text(model, &prompt, max_tokens, 0.8).await
    }
}

// Popular Ollama models for text generation
pub const RECOMMENDED_OLLAMA_MODELS: &[&str] = &[
    "llama3.2",          // Meta's latest small model (3B)
    "llama3.2:1b",       // Very fast, 1B parameters
    "gemma2:2b",         // Google's efficient 2B model
    "phi3:mini",         // Microsoft's 3.8B model
    "qwen2:1.5b",        // Alibaba's efficient model
    "codellama:7b",      // Good for structured text
    "mistral:7b",        // High quality 7B model
    "llama3.1:8b",       // Meta's 8B model
    "gemma2:9b",         // Google's 9B model
    "qwen2:7b",          // Alibaba's 7B model
];

pub fn get_ollama_recommendation(book_size: &crate::cli::BookSize) -> &'static str {
    match book_size {
        crate::cli::BookSize::ShortStory => "llama3.2:1b",      // Fast for short content
        crate::cli::BookSize::Short => "gemma2:2b",             // Good balance
        crate::cli::BookSize::Medium => "phi3:mini",            // Better quality
        crate::cli::BookSize::Large => "mistral:7b",            // High quality
        crate::cli::BookSize::VeryLarge => "llama3.1:8b",       // Best quality
        crate::cli::BookSize::Unlimited => "llama3.1:8b",       // Consistent quality
    }
}

pub fn get_download_instructions(model: &str) -> String {
    format!(
        "To download this model, run:\n\
        ollama pull {}\n\n\
        To see all available models:\n\
        ollama list\n\n\
        To install Ollama, visit: https://ollama.ai",
        model
    )
}