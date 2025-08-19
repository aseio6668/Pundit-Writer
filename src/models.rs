use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION}};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::time::Duration;
use crate::content::{ContentType, WritingAdjustments, StructuredOutline, StructuredPrompt, PromptContext, OutputFormat};

#[derive(Serialize)]
struct HuggingFaceRequest {
    inputs: String,
    parameters: HuggingFaceParameters,
}

#[derive(Serialize)]
struct HuggingFaceParameters {
    max_new_tokens: u32,
    temperature: f32,
    repetition_penalty: f32,
    return_full_text: bool,
    do_sample: bool,
    top_p: f32,
    top_k: u32,
}

#[derive(Deserialize)]
struct HuggingFaceResponse {
    generated_text: Option<String>,
}

pub struct HuggingFaceClient {
    client: Client,
    api_key: Option<String>,
    model: String,
    base_url: String,
}

impl HuggingFaceClient {
    pub fn new(model: String, api_key: Option<String>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        
        if let Some(ref key) = api_key {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", key))
                    .map_err(|e| anyhow!("Invalid API key format: {}", e))?,
            );
        }
        
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;
        
        Ok(Self {
            client,
            api_key,
            model,
            base_url: "https://api-inference.huggingface.co/models".to_string(),
        })
    }
    
    pub async fn generate_text(&self, prompt: &str, max_tokens: u32, temperature: f32) -> Result<String> {
        let url = format!("{}/{}", self.base_url, self.model);
        
        
        // Simplified request format that works better with HF Inference API
        let request = serde_json::json!({
            "inputs": prompt,
            "parameters": {
                "max_new_tokens": max_tokens,
                "temperature": temperature,
                "do_sample": true,
                "return_full_text": false
            }
        });
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Request failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            
            let error_msg = if status == 401 {
                format!("Authentication failed (401). Please check your API key or try a model that doesn't require authentication like 'gpt2'. Error: {}", error_text)
            } else if status == 404 {
                let mut msg = format!("Model '{}' not found (404). ", self.model);
                if let Some(suggestion) = suggest_model_correction(&self.model) {
                    msg.push_str(&format!("Did you mean '{}'? ", suggestion));
                }
                msg.push_str("Try: gpt2, distilgpt2, google/flan-t5-small, or facebook/opt-125m");
                msg
            } else if status == 429 {
                format!("Rate limit exceeded (429). Please wait a moment and try again. Error: {}", error_text)
            } else if status == 503 {
                format!("Model is currently unavailable (503). Try again later or use a different model. Error: {}", error_text)
            } else {
                format!("API request failed with status {}: {}", status, error_text)
            };
            
            return Err(anyhow!(error_msg));
        }
        
        let response_text = response.text().await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;
        
        // Parse the HF Inference API response format
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json) => {
                // HF API can return different formats, try to extract text
                if let Some(array) = json.as_array() {
                    if let Some(first) = array.first() {
                        if let Some(generated_text) = first.get("generated_text") {
                            if let Some(text) = generated_text.as_str() {
                                return Ok(text.to_string());
                            }
                        }
                    }
                }
                
                // Sometimes it's a direct object
                if let Some(generated_text) = json.get("generated_text") {
                    if let Some(text) = generated_text.as_str() {
                        return Ok(text.to_string());
                    }
                }
                
                // If it's just a string response
                if let Some(text) = json.as_str() {
                    return Ok(text.to_string());
                }
                
                Err(anyhow!("Unexpected response format: {}", response_text))
            }
            Err(_) => {
                // Maybe it's just plain text
                Ok(response_text)
            }
        }
    }
    
    pub async fn generate_chapter(&self, context: &str, chapter_outline: &str, target_words: usize) -> Result<String> {
        let tokens_per_word = 1.3; // Approximate ratio
        let max_tokens = (target_words as f32 * tokens_per_word) as u32;
        
        let prompt = format!(
            "Context: {}\n\nChapter Outline: {}\n\nChapter Content:",
            context, chapter_outline
        );
        
        self.generate_text(&prompt, max_tokens, 0.8).await
    }
    
    pub async fn generate_outline(&self, genre: &str, style: &str, premise: &str, num_chapters: usize) -> Result<String> {
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
        
        self.generate_text(&prompt, 1000, 0.7).await
    }
    
    pub async fn check_model_availability(&self) -> Result<bool> {
        // For free inference API, we'll just try a simple generation instead of checking availability
        let test_prompt = "Hello";
        match self.generate_text(test_prompt, 10, 0.7).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false), // Model might still work, just return false for warning
        }
    }
    
    // New methods for different content types
    pub async fn generate_content_outline(&self, content_type: &ContentType, genre: &str, style: &str, premise: &str, num_sections: usize) -> Result<String> {
        let (content_name, section_name) = match content_type {
            ContentType::Book => ("book", "chapter"),
            ContentType::Screenplay => ("screenplay", "scene"),
            ContentType::Play => ("stage play", "act"),
            ContentType::TvScript => ("TV script", "episode"),
            ContentType::AudioScript => ("audio script", "segment"),
            ContentType::GameScript => ("game script", "interaction"),
            ContentType::Document => ("document", "section"),
            ContentType::TechnicalDoc => ("technical documentation", "section"),
            ContentType::WhitePaper => ("white paper", "section"),
            ContentType::ResearchReport => ("research report", "section"),
            ContentType::Poetry => ("poetry collection", "poem"),
            ContentType::InteractiveFiction => ("interactive fiction", "chapter"),
            ContentType::Journal => ("journal", "entry"),
            ContentType::Memoir => ("memoir", "chapter"),
            ContentType::MarketingAd => ("marketing content", "section"),
            ContentType::PressRelease => ("press release", "section"),
            ContentType::MediaKit => ("media kit", "section"),
            ContentType::BlogPost => ("blog post", "section"),
            ContentType::SeoArticle => ("SEO article", "section"),
            ContentType::StrategicDoc => ("strategic document", "section"),
            ContentType::PlanningDoc => ("planning document", "section"),
            ContentType::MeetingNotes => ("meeting notes", "section"),
            ContentType::MeetingSummary => ("meeting summary", "section"),
        };
        
        let specific_instructions = match content_type {
            ContentType::Screenplay => "Include scene headings like 'INT. LOCATION - DAY' or 'EXT. LOCATION - NIGHT'.",
            ContentType::Play => "Include act divisions and stage directions.",
            ContentType::TvScript => "Include episode structure with act breaks.",
            ContentType::AudioScript => "Include audio cues and timing considerations.",
            ContentType::GameScript => "Include character interactions and dialogue choices.",
            ContentType::Document => "Include logical section flow and key topics.",
            ContentType::Book => "Include character development and plot progression.",
            ContentType::TechnicalDoc => "Include technical specifications and step-by-step instructions.",
            ContentType::WhitePaper => "Include executive summary, analysis, and recommendations.",
            ContentType::ResearchReport => "Include methodology, findings, and conclusions.",
            ContentType::Poetry => "Include thematic progression and emotional journey.",
            ContentType::InteractiveFiction => "Include choice points and branching narratives.",
            ContentType::Journal => "Include personal reflections and chronological entries.",
            ContentType::Memoir => "Include life events and personal insights.",
            ContentType::MarketingAd => "Include compelling headlines and call-to-action elements.",
            ContentType::PressRelease => "Include newsworthy angles and key messages.",
            ContentType::MediaKit => "Include brand information and media assets.",
            ContentType::BlogPost => "Include engaging content and reader value.",
            ContentType::SeoArticle => "Include keyword optimization and search intent.",
            ContentType::StrategicDoc => "Include strategic objectives and implementation plans.",
            ContentType::PlanningDoc => "Include timelines, resources, and deliverables.",
            ContentType::MeetingNotes => "Include discussion points and action items.",
            ContentType::MeetingSummary => "Include key decisions and next steps.",
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
        
        self.generate_text(&prompt, 1000, 0.7).await
    }
    
    pub async fn generate_content_section(&self, content_type: &ContentType, genre: &str, context: &str, section_outline: &str, target_words: usize) -> Result<String> {
        let tokens_per_word = 1.3;
        let max_tokens = (target_words as f32 * tokens_per_word) as u32;
        
        // Get genre-specific writing adjustments
        let writing_adjustments = WritingAdjustments::for_genre(genre);
        
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
            ContentType::TechnicalDoc => {
                "\n\nFormat as technical documentation with:\n\
                - Clear step-by-step instructions\n\
                - Code examples and screenshots (as placeholders)\n\
                - Warning and note boxes\n\
                - Professional technical language"
            },
            ContentType::WhitePaper => {
                "\n\nFormat as a white paper with:\n\
                - Executive summary style\n\
                - Data-driven arguments\n\
                - Professional business language\n\
                - Charts and graphs (as placeholders)"
            },
            ContentType::ResearchReport => {
                "\n\nFormat as a research report with:\n\
                - Academic/professional tone\n\
                - Methodology and findings sections\n\
                - Citations and references\n\
                - Tables and data analysis"
            },
            ContentType::Poetry => {
                "\n\nFormat as poetry with:\n\
                - Verse structure\n\
                - Rhythm and meter consideration\n\
                - Metaphor and imagery\n\
                - Emotional resonance"
            },
            ContentType::InteractiveFiction => {
                "\n\nFormat as interactive fiction with:\n\
                - Second person perspective\n\
                - Choice points and branching\n\
                - Immersive descriptions\n\
                - Player agency markers"
            },
            ContentType::Journal | ContentType::Memoir => {
                "\n\nFormat as personal writing with:\n\
                - First person perspective\n\
                - Reflective tone\n\
                - Personal anecdotes\n\
                - Emotional authenticity"
            },
            ContentType::MarketingAd => {
                "\n\nFormat as marketing content with:\n\
                - Compelling headlines\n\
                - Call-to-action phrases\n\
                - Benefit-focused language\n\
                - Persuasive tone"
            },
            ContentType::PressRelease => {
                "\n\nFormat as a press release with:\n\
                - News headline format\n\
                - Who, what, when, where, why structure\n\
                - Quote sections\n\
                - Professional journalism style"
            },
            ContentType::MediaKit => {
                "\n\nFormat as media kit content with:\n\
                - Brand overview sections\n\
                - Key statistics and facts\n\
                - High-level summaries\n\
                - Media-friendly language"
            },
            ContentType::BlogPost => {
                "\n\nFormat as a blog post with:\n\
                - Engaging introductions\n\
                - Subheadings for readability\n\
                - Conversational tone\n\
                - Actionable takeaways"
            },
            ContentType::SeoArticle => {
                "\n\nFormat as SEO-optimized content with:\n\
                - Keyword integration\n\
                - Clear structure with headers\n\
                - Informative and valuable content\n\
                - Reader-focused approach"
            },
            ContentType::StrategicDoc | ContentType::PlanningDoc => {
                "\n\nFormat as strategic documentation with:\n\
                - Clear objectives and goals\n\
                - Action items and timelines\n\
                - Risk assessments\n\
                - Professional business language"
            },
            ContentType::MeetingNotes | ContentType::MeetingSummary => {
                "\n\nFormat as meeting documentation with:\n\
                - Attendee lists\n\
                - Key discussion points\n\
                - Action items and owners\n\
                - Clear, concise summaries"
            },
        };
        
        let prompt = format!(
            "Context: {}\n\nSection Outline: {}\n\n{}\n\nSection Content:{}\n\n",
            context, section_outline, writing_adjustments.get_style_instructions(), formatting_instructions
        );
        
        self.generate_text(&prompt, max_tokens, 0.8).await
    }

    pub async fn generate_structured_outline(&self, content_type: &ContentType, genre: &str, style: &str, premise: &str, num_sections: usize, target_audience: &str) -> Result<StructuredOutline> {
        let context = PromptContext {
            content_type: *content_type,
            genre: genre.to_string(),
            style: style.to_string(),
            target_audience: target_audience.to_string(),
            previous_content: None,
            current_section: None,
            total_sections: Some(num_sections),
        };

        let structured_prompt = StructuredPrompt::new_outline_generation(context, num_sections);
        
        let json_schema = create_outline_json_schema(content_type, num_sections);
        let request = format!("{}\n\nJSON SCHEMA:\n{}", 
            structured_prompt.to_formatted_prompt(premise), json_schema);

        let response = self.generate_text(&request, 2000, 0.7).await?;
        
        // Try to parse JSON response
        match StructuredOutline::from_json(&response) {
            Ok(outline) => Ok(outline),
            Err(_) => {
                // Fallback to creating a basic outline if JSON parsing fails
                println!("⚠️  JSON parsing failed, creating fallback structured outline");
                Ok(StructuredOutline::new(
                    format!("Generated {}", content_type.to_string()),
                    premise.to_string(),
                    genre.to_string(),
                    target_audience.to_string(),
                    num_sections,
                    *content_type
                ))
            }
        }
    }

    pub async fn generate_with_structured_prompt(&self, structured_prompt: &StructuredPrompt, request: &str) -> Result<String> {
        let formatted_prompt = structured_prompt.to_formatted_prompt(request);
        
        let max_tokens = match structured_prompt.output_format {
            OutputFormat::Json => 2000,
            OutputFormat::PlainText => 3000,
            OutputFormat::Markdown => 3000,
            OutputFormat::Structured => 2500,
        };

        self.generate_text(&formatted_prompt, max_tokens, 0.8).await
    }
}

// Popular models for text generation (tested working models)
pub const RECOMMENDED_MODELS: &[&str] = &[
    "gpt2",
    "gpt2-medium",
    "gpt2-large", 
    "gpt2-xl",
    "distilgpt2",
    "microsoft/DialoGPT-small",
    "microsoft/DialoGPT-medium",
    "microsoft/DialoGPT-large", 
    "EleutherAI/gpt-neo-125M",
    "EleutherAI/gpt-neo-1.3B",
    "EleutherAI/gpt-neo-2.7B",
    "EleutherAI/gpt-j-6B",
    "google/flan-t5-small",
    "google/flan-t5-base",
    "google/flan-t5-large",
    "facebook/opt-125m",
    "facebook/opt-350m",
    "facebook/opt-1.3b",
    "bigscience/bloom-560m",
    "bigscience/bloom-1b1",
    "facebook/blenderbot-400M-distill",
];

pub fn suggest_model_correction(model: &str) -> Option<&'static str> {
    let model_lower = model.to_lowercase();
    
    // Common GPT-2 typos
    if model_lower == "gp2" { return Some("gpt2"); }
    if model_lower == "gp2-medium" { return Some("gpt2-medium"); }
    if model_lower == "gp2-large" { return Some("gpt2-large"); }
    if model_lower == "gp2-xl" { return Some("gpt2-xl"); }
    if model_lower == "gt2" || model_lower == "gt-2" { return Some("gpt2"); }
    
    // Common typos and corrections
    if model_lower.contains("eleutharai") || model_lower.contains("eleutherai") {
        if model_lower.contains("1.3") {
            return Some("EleutherAI/gpt-neo-1.3B");
        } else if model_lower.contains("2.7") {
            return Some("EleutherAI/gpt-neo-2.7B");
        } else if model_lower.contains("125") {
            return Some("EleutherAI/gpt-neo-125M");
        } else if model_lower.contains("6b") || model_lower.contains("j") {
            return Some("EleutherAI/gpt-j-6B");
        }
    }
    
    if model_lower.contains("dialogpt") || model_lower.contains("dialgogt") {
        if model_lower.contains("large") {
            return Some("microsoft/DialoGPT-large");
        } else if model_lower.contains("medium") {
            return Some("microsoft/DialoGPT-medium");
        } else if model_lower.contains("small") {
            return Some("microsoft/DialoGPT-small");
        }
    }
    
    if model_lower.contains("flan") {
        if model_lower.contains("large") {
            return Some("google/flan-t5-large");
        } else if model_lower.contains("base") {
            return Some("google/flan-t5-base");
        } else if model_lower.contains("small") {
            return Some("google/flan-t5-small");
        }
    }
    
    None
}

pub fn get_model_recommendation(book_size: &crate::cli_types::BookSize) -> &'static str {
    match book_size {
        crate::cli_types::BookSize::ShortStory | crate::cli_types::BookSize::Short => "gpt2",
        crate::cli_types::BookSize::Medium => "microsoft/DialoGPT-medium",
        crate::cli_types::BookSize::Large => "microsoft/DialoGPT-large",
        crate::cli_types::BookSize::VeryLarge | crate::cli_types::BookSize::Epic | crate::cli_types::BookSize::Unlimited => "microsoft/DialoGPT-large",
    }
}

pub fn create_outline_json_schema(content_type: &ContentType, num_sections: usize) -> String {
    let section_name = match content_type {
        ContentType::Book | ContentType::Memoir | ContentType::InteractiveFiction => "chapters",
        ContentType::Screenplay | ContentType::Play => "scenes",
        ContentType::TvScript => "episodes",
        ContentType::AudioScript => "segments",
        ContentType::GameScript => "interactions",
        _ => "sections",
    };

    format!(r#"{{
  "title": "string - The title of the content",
  "premise": "string - Brief description of the main premise",
  "genre": "string - The genre",
  "target_audience": "string - Target audience description",
  "themes": ["array of strings - Main themes"],
  "sections": [
    {{
      "number": "integer - Section number (1 to {})",
      "title": "string - Section title",
      "description": "string - Detailed description of this section",
      "key_events": ["array of strings - Key events or points"],
      "target_words": "integer - Target word count (recommended: 2000-5000)",
      "section_type": "string - Type of section",
      "character_focus": ["optional array of strings - Main characters"],
      "setting": "optional string - Setting description"
    }}
  ],
  "characters": [
    {{
      "name": "string - Character name",
      "role": "string - Character role/archetype",
      "description": "string - Character description",
      "traits": ["array of strings - Key character traits"]
    }}
  ],
  "settings": [
    {{
      "name": "string - Setting name",
      "description": "string - Setting description",
      "time_period": "optional string - Time period",
      "mood": "optional string - Mood/atmosphere"
    }}
  ]
}}"#, num_sections)
}