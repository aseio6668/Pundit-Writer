use anyhow::Result;
use crate::intelligent_pivot_engine::{IntelligentPivotEngine, PivotResult};
use crate::cli_types::{Genre, WritingStyle};
use crate::content::Content;
use crate::ollama::OllamaClient;

pub struct FlowAwareWriter {
    pivot_engine: IntelligentPivotEngine,
    ollama_client: OllamaClient,
    intervention_count: u32,
    gentle_nudge_count: u32,
    total_generations: u32,
}

impl FlowAwareWriter {
    pub fn new(ollama_url: &str) -> Result<Self> {
        Ok(Self {
            pivot_engine: IntelligentPivotEngine::new(),
            ollama_client: OllamaClient::new(ollama_url.to_string())?,
            intervention_count: 0,
            gentle_nudge_count: 0,
            total_generations: 0,
        })
    }

    /// Enhanced content generation that monitors flow and intervenes when needed
    pub async fn generate_with_flow_awareness(
        &mut self,
        base_prompt: &str,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content,
        chapter_context: &str,
        max_tokens: Option<usize>
    ) -> Result<String> {
        self.total_generations += 1;
        
        // First, try normal generation
        let initial_response = self.generate_content(&base_prompt, model, max_tokens).await?;
        
        // Check if the generated content needs intervention
        let evaluation = self.pivot_engine.evaluate_content_flow(
            &initial_response,
            chapter_context,
            genre,
            style,
            current_content
        )?;

        if !evaluation.should_intervene {
            // Content is fine, return as is
            println!("✅ Content flow is healthy");
            return Ok(initial_response);
        }

        // Content needs intervention
        println!("🔄 Flow intervention triggered: {}", evaluation.explanation);
        
        if evaluation.confidence_level > 0.8 {
            // High confidence - do full intervention
            return self.perform_full_intervention(evaluation, model, genre, style, current_content, max_tokens).await;
        } else if evaluation.confidence_level > 0.5 {
            // Medium confidence - try gentle nudge first
            return self.try_gentle_nudge(&initial_response, model, genre, style, max_tokens).await;
        } else {
            // Low confidence - just use the original content with a warning
            println!("⚠️ Low confidence intervention skipped");
            return Ok(initial_response);
        }
    }

    async fn perform_full_intervention(
        &mut self,
        evaluation: PivotResult,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content,
        max_tokens: Option<usize>
    ) -> Result<String> {
        self.intervention_count += 1;
        
        println!("🎯 Performing full narrative intervention (#{}) - Confidence: {:.2}", 
                 self.intervention_count, evaluation.confidence_level);

        if let Some(intervention_prompt) = evaluation.intervention_prompt {
            let intervened_content = self.generate_content(&intervention_prompt, model, max_tokens).await?;
            
            // Check if the intervention improved things
            let post_check = self.pivot_engine.evaluate_content_flow(
                &intervened_content,
                "post-intervention check",
                genre,
                style,
                current_content
            )?;

            if !post_check.should_intervene {
                println!("✅ Intervention successful - flow improved");
                return Ok(intervened_content);
            } else if post_check.confidence_level < evaluation.confidence_level {
                println!("🔄 Intervention partially successful - reduced complexity");
                return Ok(intervened_content);
            } else {
                println!("⚠️ Intervention didn't improve flow - using simplified approach");
                return self.generate_simplified_content(model, genre, style, max_tokens).await;
            }
        } else {
            println!("⚠️ No intervention prompt generated - using gentle approach");
            return self.try_gentle_nudge("", model, genre, style, max_tokens).await;
        }
    }

    async fn try_gentle_nudge(
        &mut self,
        problematic_content: &str,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        max_tokens: Option<usize>
    ) -> Result<String> {
        self.gentle_nudge_count += 1;
        
        println!("💫 Applying gentle narrative nudge (#{}) to improve flow", self.gentle_nudge_count);
        
        let nudge_prompt = self.pivot_engine.create_gentle_nudge(problematic_content);
        let nudged_content = self.generate_content(&nudge_prompt, model, max_tokens).await?;
        
        // Quick check - if still problematic, try one more approach
        if self.pivot_engine.needs_immediate_intervention(&nudged_content)? {
            println!("🔄 Gentle nudge insufficient - trying simplified generation");
            return self.generate_simplified_content(model, genre, style, max_tokens).await;
        }
        
        println!("✅ Gentle nudge successful");
        Ok(nudged_content)
    }

    async fn generate_simplified_content(
        &mut self,
        model: &str,
        genre: &Genre,
        style: &WritingStyle,
        max_tokens: Option<usize>
    ) -> Result<String> {
        println!("🎯 Generating simplified content with clear direction");
        
        let simplified_prompt = format!(
            "Write a clear, simple, and engaging section in the {} genre using {} style. \
            Focus on concrete details, clear actions, and straightforward narrative. \
            Avoid complex logical puzzles or convoluted explanations. \
            Keep the writing accessible and momentum-driven.\n\n\
            Write with clarity and confidence:",
            format!("{:?}", genre),
            format!("{:?}", style)
        );

        let simplified_content = self.generate_content(&simplified_prompt, model, max_tokens).await?;
        
        // This should be clean by design, but let's verify
        let final_check = self.pivot_engine.needs_immediate_intervention(&simplified_content)?;
        if final_check {
            println!("⚠️ Even simplified content is complex - using fallback");
            return Ok(format!(
                "The story continues with renewed clarity and purpose. \
                Sometimes the best approach is to step back and let the narrative \
                flow naturally, focusing on what truly matters in this moment.\n\n\
                [The narrative takes a fresh direction, freed from unnecessary complexity.]"
            ));
        }
        
        Ok(simplified_content)
    }

    async fn generate_content(&self, prompt: &str, model: &str, max_tokens: Option<usize>) -> Result<String> {
        // Use existing Ollama client to generate content
        self.ollama_client.generate_text(
            model,
            prompt,
            max_tokens.unwrap_or(1000) as i32,
            0.8, // temperature
        ).await
    }

    /// Get statistics about flow interventions
    pub fn get_flow_stats(&self) -> String {
        let intervention_rate = if self.total_generations > 0 {
            (self.intervention_count + self.gentle_nudge_count) as f32 / self.total_generations as f32 * 100.0
        } else {
            0.0
        };

        format!(
            "Flow Stats: {} total generations, {} full interventions, {} gentle nudges ({:.1}% intervention rate). {}",
            self.total_generations,
            self.intervention_count,
            self.gentle_nudge_count,
            intervention_rate,
            self.pivot_engine.get_flow_summary()
        )
    }

    /// Manually trigger flow check on existing content
    pub fn check_content_flow(
        &mut self,
        content: &str,
        chapter_context: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content
    ) -> Result<PivotResult> {
        self.pivot_engine.evaluate_content_flow(content, chapter_context, genre, style, current_content)
    }

    /// Rate a previous intervention's success
    pub fn rate_intervention(&mut self, intervention_id: &str, success_score: f32, readability_improvement: f32) {
        self.pivot_engine.rate_intervention_success(intervention_id, success_score, readability_improvement);
    }

    /// Get detailed intervention statistics
    pub fn get_detailed_stats(&self) -> std::collections::HashMap<String, f32> {
        let mut stats = self.pivot_engine.get_intervention_stats();
        stats.insert("total_generations".to_string(), self.total_generations as f32);
        stats.insert("intervention_count".to_string(), self.intervention_count as f32);
        stats.insert("gentle_nudge_count".to_string(), self.gentle_nudge_count as f32);
        stats
    }
}

/// Integration helper for existing writer functions
pub async fn enhance_generation_with_flow_awareness(
    content_to_check: &str,
    base_prompt: &str,
    model: &str,
    genre: &Genre,
    style: &WritingStyle,
    current_content: &Content,
    chapter_context: &str,
    ollama_url: &str
) -> Result<String> {
    let mut flow_writer = FlowAwareWriter::new(ollama_url)?;
    
    flow_writer.generate_with_flow_awareness(
        base_prompt,
        model,
        genre,
        style,
        current_content,
        chapter_context,
        Some(1000)
    ).await
}

/// Check if content should trigger an intervention without generating new content
pub fn should_intervene_on_content(
    content: &str,
    chapter_context: &str,
    genre: &Genre,
    style: &WritingStyle,
    current_content: &Content
) -> Result<bool> {
    let mut pivot_engine = IntelligentPivotEngine::new();
    let evaluation = pivot_engine.evaluate_content_flow(content, chapter_context, genre, style, current_content)?;
    Ok(evaluation.should_intervene && evaluation.confidence_level > 0.6)
}