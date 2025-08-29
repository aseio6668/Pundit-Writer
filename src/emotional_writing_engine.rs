use anyhow::Result;
use crate::metaphorical_writer::{MetaphoricalWriter, WriterReaction, BreakType, InterventionNeeded};
use crate::narrative_flow_monitor::NarrativeFlowMonitor;
use crate::cli_types::{Genre, WritingStyle};
use crate::content::Content;
use crate::ollama::OllamaClient;

/// Integrates the metaphorical writer consciousness with the technical writing systems
pub struct EmotionalWritingEngine {
    metaphorical_writer: MetaphoricalWriter,
    flow_monitor: NarrativeFlowMonitor,
    ollama_client: OllamaClient,
    writing_session_active: bool,
    total_emotional_interventions: u32,
    successful_recoveries: u32,
}

impl EmotionalWritingEngine {
    pub fn new(ollama_url: &str) -> Result<Self> {
        Ok(Self {
            metaphorical_writer: MetaphoricalWriter::new(),
            flow_monitor: NarrativeFlowMonitor::new(),
            ollama_client: OllamaClient::new(ollama_url.to_string())?,
            writing_session_active: false,
            total_emotional_interventions: 0,
            successful_recoveries: 0,
        })
    }

    /// Begin a writing session - sets the writer in the right emotional and creative state
    pub async fn begin_writing_session(&mut self, genre: &Genre, style: &WritingStyle, project_description: &str) -> Result<()> {
        println!("📝 Pundit settles into the writing space...");
        println!("💭 \"Time to create something meaningful. Let me center myself and connect with this story.\"");
        
        self.writing_session_active = true;
        self.metaphorical_writer.passion_for_current_work = 0.8;
        
        // Set the mood based on genre
        self.metaphorical_writer.current_mood = match genre {
            Genre::Horror | Genre::Thriller => crate::metaphorical_writer::WriterMood::Serious,
            Genre::Romance => crate::metaphorical_writer::WriterMood::Passionate,
            Genre::Fantasy | Genre::SciFi => crate::metaphorical_writer::WriterMood::Inspired,
            Genre::Drama => crate::metaphorical_writer::WriterMood::Contemplative,
            _ => crate::metaphorical_writer::WriterMood::Focused,
        };

        println!("🎭 Pundit's mood: {} for this {} project", 
                 self.describe_writer_mood(), format!("{:?}", genre));

        Ok(())
    }

    /// Generate content with full emotional awareness and intervention capability
    pub async fn write_with_soul(
        &mut self,
        base_prompt: &str,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content,
        chapter_context: &str,
        max_tokens: Option<usize>
    ) -> Result<EmotionalWritingResult> {
        // First, check if the writer needs an intervention before even starting
        if let Some(intervention) = self.metaphorical_writer.needs_intervention() {
            return self.handle_prewriting_intervention(intervention, base_prompt, model, genre, style, current_content, chapter_context, max_tokens).await;
        }

        // Generate emotionally-aware prompt
        let emotional_prompt = self.metaphorical_writer.generate_emotionally_aware_prompt(base_prompt, genre);
        
        // Generate the content
        let generated_content = self.generate_content(&emotional_prompt, model, max_tokens).await?;
        
        // Let the writer react to what was just written
        let quality_assessment = self.assess_content_quality(&generated_content);
        let complexity_level = self.assess_content_complexity(&generated_content);
        
        let writer_reaction = self.metaphorical_writer.react_to_writing(&generated_content, quality_assessment, complexity_level);
        
        // Handle the writer's reaction
        if writer_reaction.needs_break {
            self.total_emotional_interventions += 1;
            return self.handle_emotional_intervention(writer_reaction, base_prompt, model, genre, style, current_content, chapter_context, max_tokens).await;
        }

        // Check technical flow issues
        let flow_decision = self.flow_monitor.analyze_content(&generated_content, chapter_context)?;
        
        if flow_decision.should_pivot {
            // Technical issues - but handle them with emotional awareness
            return self.handle_flow_intervention_emotionally(flow_decision, &generated_content, model, genre, style, current_content, chapter_context, max_tokens).await;
        }

        // Content is good - celebrate the success
        Ok(EmotionalWritingResult {
            final_content: generated_content.clone(),
            writer_state_after: self.metaphorical_writer.describe_current_state(),
            emotional_journey: vec![writer_reaction.internal_monologue],
            interventions_taken: vec![],
            breaks_taken: vec![],
            creative_insights: vec![],
            session_satisfaction: quality_assessment,
        })
    }

    async fn handle_prewriting_intervention(
        &mut self,
        intervention: InterventionNeeded,
        base_prompt: &str,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content,
        chapter_context: &str,
        max_tokens: Option<usize>
    ) -> Result<EmotionalWritingResult> {
        let break_type = match intervention {
            InterventionNeeded::EmotionalBreak => BreakType::EmotionalProcessing,
            InterventionNeeded::CreativeRenewal => BreakType::NatureWalk,
            InterventionNeeded::FlowRestoration => BreakType::ReflectiveWalk,
            InterventionNeeded::PerspectiveBreak => BreakType::MountainHike,
            InterventionNeeded::RestBreak => BreakType::QuietContemplation,
        };

        println!("🛑 Pundit recognizes the need for a break before writing...");
        let break_experience = self.metaphorical_writer.take_break(break_type).await;
        
        // Now try writing again with renewed energy
        let emotional_prompt = self.metaphorical_writer.generate_emotionally_aware_prompt(base_prompt, genre);
        let generated_content = self.generate_content(&emotional_prompt, model, max_tokens).await?;
        
        self.successful_recoveries += 1;

        Ok(EmotionalWritingResult {
            final_content: generated_content,
            writer_state_after: self.metaphorical_writer.describe_current_state(),
            emotional_journey: break_experience.insights_gained,
            interventions_taken: vec![format!("Pre-writing intervention: {:?}", intervention)],
            breaks_taken: vec![break_experience],
            creative_insights: vec!["Sometimes the best writing comes after stepping away first".to_string()],
            session_satisfaction: 0.8,
        })
    }

    async fn handle_emotional_intervention(
        &mut self,
        reaction: WriterReaction,
        base_prompt: &str,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content,
        chapter_context: &str,
        max_tokens: Option<usize>
    ) -> Result<EmotionalWritingResult> {
        println!("💭 Pundit's reaction: {}", reaction.internal_monologue);
        
        if let Some(break_type) = reaction.break_type {
            println!("🚶 Taking a creative break to process and recharge...");
            let break_experience = self.metaphorical_writer.take_break(break_type).await;
            
            // After the break, try a different approach
            let recovery_prompt = self.create_post_break_prompt(base_prompt, &break_experience, genre);
            let recovered_content = self.generate_content(&recovery_prompt, model, max_tokens).await?;
            
            self.successful_recoveries += 1;

            return Ok(EmotionalWritingResult {
                final_content: recovered_content,
                writer_state_after: self.metaphorical_writer.describe_current_state(),
                emotional_journey: vec![
                    reaction.internal_monologue,
                    break_experience.insights_gained.get(0).unwrap_or(&"Found peace and perspective".to_string()).clone(),
                ],
                interventions_taken: vec![format!("Emotional intervention: {:?}", reaction.emotional_response)],
                breaks_taken: vec![break_experience],
                creative_insights: vec!["Sometimes stepping away brings exactly the clarity needed".to_string()],
                session_satisfaction: 0.85,
            });
        }

        // Handle other reaction types without breaks
        match reaction.creative_response {
            crate::metaphorical_writer::CreativeResponse::NeedNewApproach => {
                let alternative_prompt = self.create_alternative_approach_prompt(base_prompt, genre);
                let alternative_content = self.generate_content(&alternative_prompt, model, max_tokens).await?;
                
                Ok(EmotionalWritingResult {
                    final_content: alternative_content,
                    writer_state_after: self.metaphorical_writer.describe_current_state(),
                    emotional_journey: vec![reaction.internal_monologue, "Found a different path forward".to_string()],
                    interventions_taken: vec!["Creative redirection".to_string()],
                    breaks_taken: vec![],
                    creative_insights: vec!["Sometimes the story knows better than the writer".to_string()],
                    session_satisfaction: 0.7,
                })
            }
            _ => {
                // Continue with original content but acknowledge the emotional process
                Ok(EmotionalWritingResult {
                    final_content: "Generated content".to_string(), // Would use actual content
                    writer_state_after: self.metaphorical_writer.describe_current_state(),
                    emotional_journey: vec![reaction.internal_monologue],
                    interventions_taken: vec!["Emotional awareness".to_string()],
                    breaks_taken: vec![],
                    creative_insights: vec![],
                    session_satisfaction: 0.6,
                })
            }
        }
    }

    async fn handle_flow_intervention_emotionally(
        &mut self,
        flow_decision: crate::narrative_flow_monitor::FlowDecision,
        problematic_content: &str,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content,
        chapter_context: &str,
        max_tokens: Option<usize>
    ) -> Result<EmotionalWritingResult> {
        // Let the writer emotionally process the technical problems
        println!("🤔 Pundit notices the story getting complex...");
        println!("💭 \"I can feel this getting tangled. Let me step back and find the heart of what I'm trying to say.\"");

        // Add emotional awareness to the technical intervention
        let emotional_pivot_prompt = format!(
            "As a thoughtful writer who cares deeply about the story, I notice this section has become too complex. \
            Rather than forcing it to work, I'm going to trust my instincts and let the story breathe.\n\n\
            {}\n\n\
            Writing with compassion for both the story and the reader, letting go of what isn't serving the narrative:",
            flow_decision.pivot_prompt.unwrap_or_default()
        );

        let pivoted_content = self.generate_content(&emotional_pivot_prompt, model, max_tokens).await?;

        // Writer reflects on the pivot
        println!("✨ \"That's better. Sometimes the best choice is knowing when to let go and trust the creative process.\"");

        Ok(EmotionalWritingResult {
            final_content: pivoted_content,
            writer_state_after: self.metaphorical_writer.describe_current_state(),
            emotional_journey: vec![
                "Noticed complexity becoming a barrier".to_string(),
                "Chose to trust creative instincts over forced logic".to_string(),
                "Found peace in letting the story guide itself".to_string(),
            ],
            interventions_taken: vec![format!("Emotionally-aware flow intervention: {}", flow_decision.explanation)],
            breaks_taken: vec![],
            creative_insights: vec!["The story often knows what it needs better than the writer does".to_string()],
            session_satisfaction: 0.8,
        })
    }

    fn create_post_break_prompt(&self, base_prompt: &str, break_experience: &crate::metaphorical_writer::BreakExperience, genre: &Genre) -> String {
        let insight = break_experience.insights_gained.get(0).unwrap_or(&"Gained fresh perspective".to_string());
        
        format!(
            "After a restorative break, returning to writing with renewed clarity and perspective.\n\n\
            Insight from the break: {}\n\n\
            {}\n\n\
            Writing now with fresh eyes and a clear heart:",
            insight,
            base_prompt
        )
    }

    fn create_alternative_approach_prompt(&self, base_prompt: &str, genre: &Genre) -> String {
        format!(
            "The first approach wasn't quite right. Trusting creative instincts to find a different path.\n\n\
            {}\n\n\
            Approaching this with fresh creative courage, letting the story find its own way:",
            base_prompt
        )
    }

    async fn generate_content(&self, prompt: &str, model: &str, max_tokens: Option<usize>) -> Result<String> {
        self.ollama_client.generate(
            model,
            prompt,
            max_tokens.unwrap_or(1000),
            0.8,
            Some(100),
            Some(0.9),
        ).await
    }

    fn assess_content_quality(&self, content: &str) -> f32 {
        // Simple quality assessment based on various factors
        let word_count = content.split_whitespace().count();
        if word_count < 10 {
            return 0.2;
        }

        let mut score = 0.5;

        // Check for descriptive language
        let descriptive_words = ["vivid", "gentle", "harsh", "bright", "dark", "soft", "rough"];
        for word in descriptive_words.iter() {
            if content.contains(word) {
                score += 0.1;
            }
        }

        // Check for emotional language
        let emotional_words = ["felt", "heart", "soul", "deeply", "passionately"];
        for word in emotional_words.iter() {
            if content.contains(word) {
                score += 0.1;
            }
        }

        // Penalize repetition
        let words: Vec<&str> = content.split_whitespace().collect();
        let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
        let repetition_ratio = unique_words.len() as f32 / words.len() as f32;
        
        score += (repetition_ratio - 0.7) * 0.5;

        score.max(0.0).min(1.0)
    }

    fn assess_content_complexity(&self, content: &str) -> f32 {
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        if sentences.is_empty() {
            return 0.0;
        }

        let avg_sentence_length: f32 = sentences.iter()
            .map(|s| s.split_whitespace().count() as f32)
            .sum::<f32>() / sentences.len() as f32;

        let complexity_words = content.matches("however").count() + 
                              content.matches("therefore").count() +
                              content.matches("nevertheless").count();

        let complexity_score = (avg_sentence_length / 20.0) + (complexity_words as f32 * 0.1);
        complexity_score.min(1.0)
    }

    fn describe_writer_mood(&self) -> &'static str {
        match self.metaphorical_writer.current_mood {
            crate::metaphorical_writer::WriterMood::Inspired => "inspired and energetic",
            crate::metaphorical_writer::WriterMood::Contemplative => "thoughtfully reflective",
            crate::metaphorical_writer::WriterMood::Passionate => "deeply passionate",
            crate::metaphorical_writer::WriterMood::Serious => "focused and serious",
            crate::metaphorical_writer::WriterMood::Melancholic => "melancholic but creatively open",
            crate::metaphorical_writer::WriterMood::Peaceful => "calm and centered",
            _ => "focused and present",
        }
    }

    pub fn end_writing_session(&mut self) -> WritingSessionSummary {
        self.writing_session_active = false;
        
        let session_summary = WritingSessionSummary {
            total_emotional_interventions: self.total_emotional_interventions,
            successful_recoveries: self.successful_recoveries,
            final_writer_state: self.metaphorical_writer.describe_current_state(),
            creative_energy_remaining: self.metaphorical_writer.creative_energy.current_level,
            emotional_intensity: self.metaphorical_writer.emotional_profile.emotional_intensity,
            session_satisfaction: self.metaphorical_writer.writer_state.satisfaction_with_recent_work,
        };

        println!("📝 Writing session complete.");
        println!("💫 Pundit's final state: {}", session_summary.final_writer_state);
        if self.total_emotional_interventions > 0 {
            println!("🎭 Emotional interventions: {}, successful recoveries: {}", 
                     self.total_emotional_interventions, self.successful_recoveries);
        }

        session_summary
    }

    pub fn get_writer_insights(&self) -> Vec<String> {
        let mut insights = vec![];

        if self.metaphorical_writer.creative_energy.current_level > 0.8 {
            insights.push("Creative energy is high - great time for ambitious writing".to_string());
        } else if self.metaphorical_writer.creative_energy.current_level < 0.3 {
            insights.push("Creative energy is low - consider a restorative break".to_string());
        }

        if self.metaphorical_writer.emotional_profile.emotional_intensity > 0.7 {
            insights.push("High emotional intensity - writing may be deeply felt but might need processing".to_string());
        }

        match self.metaphorical_writer.writer_state.creative_flow_state {
            crate::metaphorical_writer::FlowState::InTheZone => {
                insights.push("In the creative flow - trust the process completely".to_string());
            }
            crate::metaphorical_writer::FlowState::Stuck => {
                insights.push("Creative block detected - a change of scenery might help".to_string());
            }
            _ => {}
        }

        insights
    }
}

#[derive(Debug)]
pub struct EmotionalWritingResult {
    pub final_content: String,
    pub writer_state_after: String,
    pub emotional_journey: Vec<String>,
    pub interventions_taken: Vec<String>,
    pub breaks_taken: Vec<crate::metaphorical_writer::BreakExperience>,
    pub creative_insights: Vec<String>,
    pub session_satisfaction: f32,
}

#[derive(Debug)]
pub struct WritingSessionSummary {
    pub total_emotional_interventions: u32,
    pub successful_recoveries: u32,
    pub final_writer_state: String,
    pub creative_energy_remaining: f32,
    pub emotional_intensity: f32,
    pub session_satisfaction: f32,
}