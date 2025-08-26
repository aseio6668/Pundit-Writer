use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::time::Duration;
use crate::content::{ContentType, WritingAdjustments, StructuredOutline, StructuredPrompt, PromptContext};
use crate::models::create_outline_json_schema;

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
    repeat_penalty: f32,
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
                repeat_penalty: 1.1, // Reduce repetition
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
            ContentType::Dictionary => ("dictionary", "entry"),
            ContentType::EducationalLesson => ("educational lesson", "module"),
            ContentType::ChildrensBook => ("children's book", "chapter"),
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
            ContentType::Dictionary => "Include word definitions, pronunciations, and etymologies.",
            ContentType::EducationalLesson => "Include learning objectives, examples, and practice exercises.",
            ContentType::ChildrensBook => "Include age-appropriate language, engaging characters, and positive themes.",
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
    
    // Enhanced method for large content generation with stronger anti-repetition
    pub async fn generate_text_large(&self, model: &str, prompt: &str, max_tokens: i32, temperature: f32) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature,
                num_predict: max_tokens,
                top_p: 0.95,         // Slightly higher for more variety
                top_k: 60,           // Higher for more diverse vocabulary
                repeat_penalty: 1.15, // Stronger anti-repetition for large content
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

    pub async fn generate_content_section(&self, model: &str, content_type: &ContentType, genre: &str, context: &str, section_outline: &str, target_words: usize) -> Result<String> {
        let max_tokens = (target_words as f32 * 1.3) as i32;
        
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
            ContentType::Dictionary => {
                "\n\nFormat as dictionary entries with:\n\
                - Word headings in bold\n\
                - Pronunciations in phonetic notation\n\
                - Multiple definitions numbered\n\
                - Etymology and origin information\n\
                - Example sentences in italics"
            },
            ContentType::EducationalLesson => {
                "\n\nFormat as educational content with:\n\
                - Clear learning objectives\n\
                - Step-by-step instructions\n\
                - Examples and practice exercises\n\
                - Age-appropriate language\n\
                - Interactive elements and assessments"
            },
            ContentType::ChildrensBook => {
                "\n\nFormat as children's literature with:\n\
                - Age-appropriate vocabulary\n\
                - Engaging character dialogue\n\
                - Descriptive but simple sentences\n\
                - Positive themes and lessons\n\
                - Illustration descriptions in [brackets]"
            },
        };
        
        let prompt = format!(
            "Context: {}\n\nSection Outline: {}\n\n{}\n\nWrite detailed content based on the outline above. Target length: {} words.{}\n\nContent:",
            context, section_outline, writing_adjustments.get_style_instructions(), target_words, formatting_instructions
        );
        
        // Use enhanced generation for large content (>5000 words per section)
        let generated_text = if target_words > 5000 {
            self.generate_text_large(model, &prompt, max_tokens, 0.8).await?
        } else {
            self.generate_text(model, &prompt, max_tokens, 0.8).await?
        };
        
        Ok(complete_incomplete_sentences(generated_text))
    }

    pub async fn generate_structured_outline(&self, model: &str, content_type: &ContentType, genre: &str, style: &str, premise: &str, num_sections: usize, target_audience: &str) -> Result<StructuredOutline> {
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

        let response = self.generate_text(model, &request, 2000, 0.7).await?;
        
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

    pub async fn generate_with_structured_prompt(&self, model: &str, structured_prompt: &StructuredPrompt, request: &str) -> Result<String> {
        let formatted_prompt = structured_prompt.to_formatted_prompt(request);
        
        let max_tokens = match structured_prompt.output_format {
            crate::content::OutputFormat::Json => 2000,
            crate::content::OutputFormat::PlainText => 3000,
            crate::content::OutputFormat::Markdown => 3000,
            crate::content::OutputFormat::Structured => 2500,
        };

        // Use enhanced generation for structured prompts
        self.generate_text_large(model, &formatted_prompt, max_tokens, 0.8).await
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

pub fn get_ollama_recommendation(book_size: &crate::cli_types::BookSize) -> &'static str {
    match book_size {
        crate::cli_types::BookSize::ShortStory => "llama3.2:1b",      // Fast for short content
        crate::cli_types::BookSize::Short => "gemma2:2b",             // Good balance
        crate::cli_types::BookSize::Medium => "phi3:mini",            // Better quality
        crate::cli_types::BookSize::Large => "mistral:7b",            // High quality
        crate::cli_types::BookSize::VeryLarge => "llama3.1:8b",       // Best quality
        crate::cli_types::BookSize::Epic => "llama3.1:8b",            // Consistent quality for epic books
        crate::cli_types::BookSize::Unlimited => "llama3.1:8b",       // Consistent quality
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

/// Complete incomplete sentences that may be cut off due to token limits
fn complete_incomplete_sentences(text: String) -> String {
    let text = text.trim();
    if text.is_empty() {
        return text.to_string();
    }
    
    // Check if the text ends with a complete sentence
    let last_char = text.chars().last().unwrap_or(' ');
    
    // If text ends with proper sentence ending punctuation, it's likely complete
    if matches!(last_char, '.' | '!' | '?' | '"' | '\'' | ')') {
        return text.to_string();
    }
    
    // For dialogue or special formatting, check if it ends reasonably
    if text.ends_with(':') || text.ends_with(';') {
        return text.to_string();
    }
    
    // If text ends mid-sentence, try to find a good stopping point
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return text.to_string();
    }
    
    // Look for the last complete sentence by working backwards
    let mut complete_text = Vec::new();
    let mut found_complete_sentence = false;
    
    for line in lines.iter().rev() {
        let line = line.trim();
        if line.is_empty() {
            complete_text.push(line);
            continue;
        }
        
        // Check if this line ends with sentence-ending punctuation
        let line_last_char = line.chars().last().unwrap_or(' ');
        if matches!(line_last_char, '.' | '!' | '?' | '"' | '\'') && !found_complete_sentence {
            complete_text.push(line);
            found_complete_sentence = true;
            break;
        }
        
        complete_text.push(line);
    }
    
    // If we found a complete sentence, use text up to that point
    if found_complete_sentence {
        complete_text.reverse();
        let result = complete_text.join("\n").trim().to_string();
        if !result.is_empty() {
            return result;
        }
    }
    
    // If no complete sentence found, try to find a paragraph break
    let paragraphs: Vec<&str> = text.split("\n\n").collect();
    if paragraphs.len() > 1 {
        // Return all complete paragraphs except the last (potentially incomplete) one
        let complete_paragraphs = &paragraphs[..paragraphs.len() - 1];
        let result = complete_paragraphs.join("\n\n").trim().to_string();
        if !result.is_empty() {
            return result;
        }
    }
    
    // Last resort: try to end at the last complete word before a reasonable cutoff
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() > 50 { // Only truncate if we have a substantial amount of text
        // Look for a good place to end within the last portion of the text
        let cutoff_point = words.len() * 4 / 5; // Use 80% of the text
        let truncated_words = &words[..cutoff_point];
        let truncated_text = truncated_words.join(" ");
        
        // Make sure we're not cutting off in the middle of dialogue or important punctuation
        if !truncated_text.contains("\"") || truncated_text.matches("\"").count() % 2 == 0 {
            return format!("{}.", truncated_text); // Add a period to make it complete
        }
    }
    
    // If all else fails, return the original text (it might be acceptable as-is)
    text.to_string()
}