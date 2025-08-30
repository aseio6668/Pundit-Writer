use anyhow::Result;
use crate::cli_types::{Genre, WritingStyle};
use crate::content::{Content, ContentType};
use crate::silent_mind_writer::{SilentMindWriter, ContemplativeState};
use crate::writer::AIClient;

/// Enhanced writer engine that integrates contemplative depth with AI generation
pub struct ContemplativeWriterEngine {
    silent_mind: SilentMindWriter,
    contemplative_sessions: u32,
    accumulated_depth: f32,
    current_contemplative_state: Option<ContemplativeState>,
}

impl ContemplativeWriterEngine {
    pub fn new() -> Self {
        Self {
            silent_mind: SilentMindWriter::new(),
            contemplative_sessions: 0,
            accumulated_depth: 0.0,
            current_contemplative_state: None,
        }
    }
    
    /// Begin a contemplative writing session
    pub async fn begin_contemplative_session(
        &mut self, 
        genre: &Genre, 
        style: &WritingStyle, 
        content: &Content
    ) -> Result<()> {
        println!("🧘 Entering contemplative preparation...");
        
        let content_type_name = match content.content_type {
            ContentType::Book => "narrative",
            ContentType::Poetry => "poetry",
            ContentType::Screenplay => "screenplay",
            ContentType::Play => "theatrical",
            ContentType::TechnicalDoc => "technical",
            ContentType::Journal => "personal reflection",
            _ => "creative writing",
        };
        
        // Enter contemplation state
        let contemplative_state = self.silent_mind.enter_contemplation(genre, style, content_type_name)?;
        
        // Display contemplation guidance to user (briefly)
        println!("   💭 {}", self.silent_mind.get_meditation_state_description());
        
        if let Some(quiet_msg) = self.get_quiet_mind_message(&contemplative_state) {
            println!("   🌅 {}", quiet_msg);
        }
        
        self.current_contemplative_state = Some(contemplative_state);
        self.contemplative_sessions += 1;
        
        // Brief pause to represent the contemplative moment (not visible to user)
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Ok(())
    }
    
    fn get_quiet_mind_message(&self, state: &ContemplativeState) -> Option<String> {
        if state.inner_silence_achieved > 0.7 {
            Some("Writing from deep inner stillness...".to_string())
        } else if state.inner_silence_achieved > 0.5 {
            Some("Settling into contemplative awareness...".to_string())
        } else {
            None
        }
    }
    
    /// Generate content with contemplative enhancement
    pub async fn generate_contemplative_content(
        &self,
        client: &AIClient,
        model: &str,
        base_prompt: &str,
        content: &Content,
        section_number: usize,
        target_words: usize,
        language: &str,
    ) -> Result<String> {
        let contemplative_state = self.current_contemplative_state.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No contemplative state active. Call begin_contemplative_session first."))?;
        
        // Create enhanced prompt with contemplative guidance
        let enhanced_prompt = self.create_enhanced_contemplative_prompt(
            base_prompt, 
            contemplative_state, 
            content, 
            section_number,
            language
        );
        
        // Generate content using the enhanced prompt
        let raw_content = match client {
            AIClient::HuggingFace(hf_client) => {
                hf_client.generate_content_section(
                    &content.content_type,
                    &content.genre,
                    &content.get_clean_context(),
                    &enhanced_prompt,
                    target_words
                ).await?
            },
            AIClient::Ollama(ollama_client) => {
                ollama_client.generate_content_section(
                    model,
                    &content.content_type,
                    &content.genre,
                    &content.get_clean_context(),
                    &enhanced_prompt,
                    target_words
                ).await?
            }
        };
        
        // Apply contemplative refinement to reduce chattiness and enhance depth
        let refined_content = self.silent_mind.refine_contemplative_output(&raw_content, contemplative_state);
        
        // Apply additional contemplative enhancements based on content type
        let final_content = self.apply_content_type_contemplation(&refined_content, &content.content_type, contemplative_state);
        
        Ok(final_content)
    }
    
    fn create_enhanced_contemplative_prompt(
        &self,
        base_prompt: &str,
        contemplative_state: &ContemplativeState,
        content: &Content,
        section_number: usize,
        language: &str,
    ) -> String {
        // Create the base contemplative prompt
        let contemplative_prompt = self.silent_mind.create_contemplative_prompt(base_prompt, contemplative_state);
        
        // Add content-specific contemplative guidance
        let content_guidance = self.generate_content_specific_guidance(&content.content_type, contemplative_state);
        
        // Add section-specific depth guidance
        let section_guidance = self.generate_section_depth_guidance(section_number, contemplative_state.depth_enhancement);
        
        // Language instruction (if not English)
        let language_instruction = if language != "English" {
            format!("\n\nIMPORTANT: Write entirely in {} language. Let the contemplative depth flow naturally through the chosen language.", language)
        } else {
            String::new()
        };
        
        // Combine all elements with contemplative flow
        format!(
            "{}\n\n--- CONTEMPLATIVE CONTENT GUIDANCE ---\n{}\n\n--- DEPTH ENHANCEMENT ---\n{}\n\n--- WRITING INSTRUCTION ---\nFrom this state of contemplative awareness, write with natural flow and depth. Let each word emerge from stillness rather than hurried thought. Avoid excessive explanation or chatty commentary - let the content speak from its own truth.{}\n\n--- AVOID ---\nExcessive use of phrases like 'well', 'you know', 'actually', 'I mean', 'basically', 'the point is', 'what I'm trying to say'. Write from clarity rather than explanation.",
            contemplative_prompt,
            content_guidance,
            section_guidance,
            language_instruction
        )
    }
    
    fn generate_content_specific_guidance(&self, content_type: &ContentType, state: &ContemplativeState) -> String {
        let depth_descriptor = if state.depth_enhancement > 1.4 {
            "profound"
        } else if state.depth_enhancement > 1.2 {
            "thoughtful"
        } else {
            "gentle"
        };
        
        match content_type {
            ContentType::Book | ContentType::InteractiveFiction => {
                format!("Write narrative with {} depth. Let characters and scenes emerge from contemplative observation rather than surface description. Show inner worlds through quiet revelation.", depth_descriptor)
            },
            ContentType::Poetry => {
                format!("Let poetry arise from the silence between thoughts. Each line should carry the essence of contemplative awareness. Write with {} stillness and natural rhythm.", depth_descriptor)
            },
            ContentType::Screenplay | ContentType::Play => {
                format!("Create dialogue that emerges from character depths rather than surface chatter. Let dramatic moments breathe with {} contemplative space.", depth_descriptor)
            },
            ContentType::Journal | ContentType::Memoir => {
                format!("Write personal reflection with {} introspective awareness. Let insights emerge naturally from quiet contemplation of experience.", depth_descriptor)
            },
            ContentType::TechnicalDoc | ContentType::Document => {
                format!("Present information with {} clarity that comes from understanding rather than mere explanation. Let knowledge flow from comprehension to reader.", depth_descriptor)
            },
            _ => {
                format!("Write with {} contemplative awareness, letting content emerge from inner understanding rather than mental construction.", depth_descriptor)
            }
        }
    }
    
    fn generate_section_depth_guidance(&self, section_number: usize, depth_enhancement: f32) -> String {
        let session_depth = self.contemplative_sessions as f32 * 0.1 + depth_enhancement;
        
        if section_number == 1 {
            format!("As the opening section, establish a contemplative foundation. Write with depth level {:.1} - let the beginning emerge from stillness rather than rushed introduction.", session_depth)
        } else if section_number <= 3 {
            format!("Continue building contemplative depth. With accumulated sessions ({}), write from enhanced awareness level {:.1}.", self.contemplative_sessions, session_depth)
        } else {
            format!("Drawing from {} contemplative sessions, write with mature depth level {:.1}. Let the content flow from established inner stillness.", self.contemplative_sessions, session_depth)
        }
    }
    
    fn apply_content_type_contemplation(&self, content: &str, content_type: &ContentType, state: &ContemplativeState) -> String {
        match content_type {
            ContentType::Poetry => self.enhance_poetic_contemplation(content, state),
            ContentType::Journal | ContentType::Memoir => self.enhance_reflective_contemplation(content, state),
            ContentType::TechnicalDoc => self.enhance_technical_clarity(content, state),
            _ => self.enhance_general_contemplation(content, state),
        }
    }
    
    fn enhance_poetic_contemplation(&self, content: &str, state: &ContemplativeState) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let enhanced_lines: Vec<String> = lines.iter()
            .map(|line| {
                if state.inner_silence_achieved > 0.7 && line.len() > 20 {
                    // Add contemplative pauses in poetry
                    line.replace(",", "...")
                        .replace("; ", "...\n")
                } else {
                    line.to_string()
                }
            })
            .collect();
        
        enhanced_lines.join("\n")
    }
    
    fn enhance_reflective_contemplation(&self, content: &str, state: &ContemplativeState) -> String {
        if state.contemplation_depth > 0.8 {
            // Add contemplative transitions for deep reflection
            content.replace("I realized", "In quiet reflection, it became clear")
                   .replace("I thought", "It occurred to me")
                   .replace("I felt", "A feeling arose")
        } else {
            content.to_string()
        }
    }
    
    fn enhance_technical_clarity(&self, content: &str, state: &ContemplativeState) -> String {
        if state.depth_enhancement > 1.3 {
            // Enhance technical writing with contemplative clarity
            content.replace("It is important to note", "Consider that")
                   .replace("You should understand", "Understanding emerges when")
                   .replace("Make sure to", "Ensure you")
        } else {
            content.to_string()
        }
    }
    
    fn enhance_general_contemplation(&self, content: &str, state: &ContemplativeState) -> String {
        let mut enhanced = content.to_string();
        
        if state.inner_silence_achieved > 0.6 {
            // Replace hurried language with contemplative alternatives
            enhanced = enhanced.replace("quickly", "steadily")
                              .replace("immediately", "naturally")
                              .replace("obviously", "clearly")
                              .replace("definitely", "certainly");
        }
        
        if state.contemplation_depth > 0.7 {
            // Add contemplative depth to observations
            enhanced = enhanced.replace(" saw ", " observed ")
                              .replace(" looked ", " regarded ")
                              .replace(" heard ", " listened to ");
        }
        
        enhanced
    }
    
    /// End contemplative session and update accumulated wisdom
    pub fn end_contemplative_session(&mut self) -> ContemplativeSessionSummary {
        let session_summary = if let Some(state) = &self.current_contemplative_state {
            ContemplativeSessionSummary {
                inner_silence_achieved: state.inner_silence_achieved,
                contemplative_depth_reached: state.contemplation_depth,
                session_number: self.contemplative_sessions,
                accumulated_wisdom: self.accumulated_depth,
                meditation_state: self.silent_mind.get_meditation_state_description(),
            }
        } else {
            ContemplativeSessionSummary::default()
        };
        
        // Update accumulated depth for future sessions
        if let Some(state) = &self.current_contemplative_state {
            self.accumulated_depth = (self.accumulated_depth + state.contemplation_depth * 0.1).min(2.0);
        }
        
        self.current_contemplative_state = None;
        session_summary
    }
    
    /// Reset for new writing project
    pub fn reset_contemplative_journey(&mut self) {
        self.silent_mind.reset_contemplation();
        self.contemplative_sessions = 0;
        self.accumulated_depth = 0.0;
        self.current_contemplative_state = None;
    }
    
    /// Get current contemplative status
    pub fn get_contemplative_status(&self) -> Option<String> {
        self.current_contemplative_state.as_ref().map(|state| {
            format!(
                "Contemplative depth: {:.1}%, Inner silence: {:.1}%, Sessions completed: {}",
                state.contemplation_depth * 100.0,
                state.inner_silence_achieved * 100.0,
                self.contemplative_sessions
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct ContemplativeSessionSummary {
    pub inner_silence_achieved: f32,
    pub contemplative_depth_reached: f32,
    pub session_number: u32,
    pub accumulated_wisdom: f32,
    pub meditation_state: String,
}

impl Default for ContemplativeSessionSummary {
    fn default() -> Self {
        Self {
            inner_silence_achieved: 0.0,
            contemplative_depth_reached: 0.0,
            session_number: 0,
            accumulated_wisdom: 0.0,
            meditation_state: "No active session".to_string(),
        }
    }
}

impl Default for ContemplativeWriterEngine {
    fn default() -> Self {
        Self::new()
    }
}