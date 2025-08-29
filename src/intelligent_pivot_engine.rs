use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::narrative_flow_monitor::{NarrativeFlowMonitor, FlowDecision, PivotStrategy, ExtractedElement};
use crate::cli_types::{Genre, WritingStyle};
use crate::content::Content;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligentPivotEngine {
    flow_monitor: NarrativeFlowMonitor,
    pivot_success_rate: f32,
    adaptive_thresholds: AdaptiveThresholds,
    creative_memory: CreativeMemory,
    intervention_history: Vec<PivotIntervention>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveThresholds {
    confusion_sensitivity: f32,
    readability_minimum: f32,
    repetition_tolerance: f32,
    complexity_ceiling: f32,
    intervention_frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeMemory {
    successful_pivots: HashMap<String, SuccessfulPivot>,
    failed_pivot_patterns: Vec<String>,
    element_reuse_success: HashMap<String, f32>,
    genre_pivot_compatibility: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessfulPivot {
    strategy_used: PivotStrategy,
    context_before: String,
    context_after: String,
    extracted_elements: Vec<ExtractedElement>,
    success_score: f32,
    reuse_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotIntervention {
    intervention_id: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    trigger_reason: String,
    content_before: String,
    strategy_applied: PivotStrategy,
    generated_prompt: String,
    success_rating: Option<f32>,
    readability_improvement: Option<f32>,
    user_satisfaction: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotResult {
    pub should_intervene: bool,
    pub intervention_prompt: Option<String>,
    pub confidence_level: f32,
    pub explanation: String,
    pub extracted_elements: Vec<ExtractedElement>,
    pub recommended_approach: Option<PivotApproach>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotApproach {
    strategy: PivotStrategy,
    explanation: String,
    creative_direction: String,
    preservation_elements: Vec<String>,
    abandonment_summary: String,
}

impl Default for IntelligentPivotEngine {
    fn default() -> Self {
        Self {
            flow_monitor: NarrativeFlowMonitor::new(),
            pivot_success_rate: 0.75,
            adaptive_thresholds: AdaptiveThresholds {
                confusion_sensitivity: 0.7,
                readability_minimum: 0.4,
                repetition_tolerance: 0.6,
                complexity_ceiling: 0.8,
                intervention_frequency: 0.3,
            },
            creative_memory: CreativeMemory {
                successful_pivots: HashMap::new(),
                failed_pivot_patterns: Vec::new(),
                element_reuse_success: HashMap::new(),
                genre_pivot_compatibility: HashMap::new(),
            },
            intervention_history: Vec::new(),
        }
    }
}

impl IntelligentPivotEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Main entry point for evaluating whether to pivot the narrative
    pub fn evaluate_content_flow(
        &mut self, 
        content: &str, 
        chapter_context: &str,
        genre: &Genre,
        style: &WritingStyle,
        current_content: &Content
    ) -> Result<PivotResult> {
        // Analyze the content flow
        let flow_decision = self.flow_monitor.analyze_content(content, chapter_context)?;
        
        if !flow_decision.should_pivot {
            return Ok(PivotResult {
                should_intervene: false,
                intervention_prompt: None,
                confidence_level: flow_decision.confidence,
                explanation: "Content flow is healthy - no intervention needed".to_string(),
                extracted_elements: Vec::new(),
                recommended_approach: None,
            });
        }

        // Determine the best pivot approach
        let approach = self.determine_optimal_approach(
            &flow_decision, 
            genre, 
            style, 
            current_content
        )?;

        // Generate intervention prompt
        let intervention_prompt = self.create_intervention_prompt(
            &approach, 
            &flow_decision.extracted_elements, 
            content,
            chapter_context
        )?;

        // Log the intervention
        self.log_intervention(&approach, content, &intervention_prompt);

        Ok(PivotResult {
            should_intervene: true,
            intervention_prompt: Some(intervention_prompt),
            confidence_level: flow_decision.confidence,
            explanation: self.explain_intervention_reasoning(&flow_decision, &approach),
            extracted_elements: flow_decision.extracted_elements,
            recommended_approach: Some(approach),
        })
    }

    fn determine_optimal_approach(
        &self,
        flow_decision: &FlowDecision,
        genre: &Genre,
        style: &WritingStyle,
        content: &Content
    ) -> Result<PivotApproach> {
        let base_strategy = flow_decision.recommended_strategy
            .as_ref()
            .unwrap_or(&PivotStrategy::ElementReuse);

        // Check if we have successful patterns for this strategy
        let refined_strategy = self.refine_strategy_from_history(base_strategy, genre);
        
        let approach = match refined_strategy {
            PivotStrategy::CompleteShift => self.create_complete_shift_approach(genre, style),
            PivotStrategy::ElementReuse => self.create_element_reuse_approach(&flow_decision.extracted_elements, genre),
            PivotStrategy::TemporalJump => self.create_temporal_jump_approach(content, genre),
            PivotStrategy::PerspectiveShift => self.create_perspective_shift_approach(content, &flow_decision.extracted_elements),
            PivotStrategy::SummaryIntegration => self.create_summary_integration_approach(&flow_decision.extracted_elements),
            PivotStrategy::GenreBlend => self.create_genre_blend_approach(genre, style),
            PivotStrategy::MetaResolution => self.create_meta_resolution_approach(&flow_decision.detected_issues),
        };

        Ok(approach)
    }

    fn create_complete_shift_approach(&self, genre: &Genre, style: &WritingStyle) -> PivotApproach {
        PivotApproach {
            strategy: PivotStrategy::CompleteShift,
            explanation: "The narrative has become too tangled to continue directly. We're making a creative leap to an entirely new direction.".to_string(),
            creative_direction: format!(
                "Shift to a completely different scene, setting, or storyline within the {} genre. Use {} style but feel free to explore new territory. This isn't a continuation - it's a fresh creative direction that can stand on its own.",
                format!("{:?}", genre),
                format!("{:?}", style)
            ),
            preservation_elements: vec!["genre consistency".to_string(), "writing style".to_string()],
            abandonment_summary: "Letting go of the current complex plot threads to avoid logical traps".to_string(),
        }
    }

    fn create_element_reuse_approach(&self, elements: &[ExtractedElement], genre: &Genre) -> PivotApproach {
        let key_elements: Vec<String> = elements.iter()
            .filter(|e| e.importance_score > 0.5)
            .map(|e| format!("{} ({})", e.content, format!("{:?}", e.element_type)))
            .collect();

        PivotApproach {
            strategy: PivotStrategy::ElementReuse,
            explanation: "Extracting valuable elements from the complex section and reusing them in a simpler context.".to_string(),
            creative_direction: format!(
                "Create a new scene or chapter that incorporates these elements: {}. Use them naturally in a fresh context without trying to resolve the previous complexity. Let the story breathe with these familiar elements in new situations.",
                key_elements.join(", ")
            ),
            preservation_elements: key_elements,
            abandonment_summary: "Abandoning the convoluted plot logic while preserving memorable characters, locations, and concepts".to_string(),
        }
    }

    fn create_temporal_jump_approach(&self, content: &Content, genre: &Genre) -> PivotApproach {
        let jump_options = vec![
            "Jump forward several days/weeks/months",
            "Flash back to an earlier, simpler time", 
            "Switch to a parallel timeline",
            "Fast-forward past the complexity"
        ];

        let selected_jump = &jump_options[0]; // Could be more sophisticated selection

        PivotApproach {
            strategy: PivotStrategy::TemporalJump,
            explanation: "Resolving complexity by jumping to a different time period.".to_string(),
            creative_direction: format!(
                "{}. The complex situation can be referenced as past events (if jumping forward) or future concerns (if jumping back), but don't try to resolve it directly. Focus on the new time period's story.",
                selected_jump
            ),
            preservation_elements: vec!["character identities".to_string(), "world setting".to_string()],
            abandonment_summary: "Moving away from the current timeline's complexities".to_string(),
        }
    }

    fn create_perspective_shift_approach(&self, content: &Content, elements: &[ExtractedElement]) -> PivotApproach {
        let character_names: Vec<String> = elements.iter()
            .filter_map(|e| match e.element_type {
                crate::narrative_flow_monitor::ElementType::CharacterName => Some(e.content.clone()),
                _ => None,
            })
            .collect();

        PivotApproach {
            strategy: PivotStrategy::PerspectiveShift,
            explanation: "Switching to a different character's viewpoint to escape the current complexity.".to_string(),
            creative_direction: format!(
                "Switch to the perspective of a different character{}. They might be aware of the complex situation but approach it differently, or be completely unaware and dealing with their own story. This new viewpoint should feel fresh and unencumbered.",
                if !character_names.is_empty() {
                    format!(" (perhaps {})", character_names.join(" or "))
                } else {
                    " (introduce a new character if needed)".to_string()
                }
            ),
            preservation_elements: vec!["world setting".to_string(), "established characters".to_string()],
            abandonment_summary: "Leaving the current character's complex thought processes and challenges".to_string(),
        }
    }

    fn create_summary_integration_approach(&self, elements: &[ExtractedElement]) -> PivotApproach {
        PivotApproach {
            strategy: PivotStrategy::SummaryIntegration,
            explanation: "Compressing the complex situation into a brief summary and moving forward.".to_string(),
            creative_direction: "Provide a concise summary of the complex situation (1-2 sentences maximum), then move the story forward with renewed clarity. The summary should capture the essence without getting bogged down in details.".to_string(),
            preservation_elements: elements.iter().map(|e| e.content.clone()).collect(),
            abandonment_summary: "Condensing complex plot threads into background information".to_string(),
        }
    }

    fn create_genre_blend_approach(&self, genre: &Genre, style: &WritingStyle) -> PivotApproach {
        let blend_suggestions = match genre {
            Genre::Fantasy => vec!["mystery elements", "slice-of-life moments", "humor"],
            Genre::Mystery => vec!["romance subplot", "adventure action", "philosophical reflection"],
            Genre::Romance => vec!["adventure elements", "mystery intrigue", "comedy"],
            Genre::SciFi => vec!["human drama", "philosophical questions", "action adventure"],
            _ => vec!["unexpected genre elements", "fresh perspective", "different tone"],
        };

        PivotApproach {
            strategy: PivotStrategy::GenreBlend,
            explanation: "Introducing fresh energy through genre blending.".to_string(),
            creative_direction: format!(
                "Introduce {} to refresh the narrative. This doesn't change the core genre but adds new dimensions. Let the story surprise both the characters and readers.",
                blend_suggestions.join(" or ")
            ),
            preservation_elements: vec!["core genre identity".to_string(), "main characters".to_string()],
            abandonment_summary: "Moving away from genre predictability".to_string(),
        }
    }

    fn create_meta_resolution_approach(&self, issues: &[crate::narrative_flow_monitor::ConfusionIndicator]) -> PivotApproach {
        PivotApproach {
            strategy: PivotStrategy::MetaResolution,
            explanation: "Acknowledging the complexity directly within the narrative.".to_string(),
            creative_direction: "Have a character recognize that the situation has become too complex, confusing, or overwhelming. They might decide to 'let it go,' 'think about it later,' or 'focus on what really matters.' This gives you permission to move past the complexity naturally.".to_string(),
            preservation_elements: vec!["character agency".to_string(), "narrative honesty".to_string()],
            abandonment_summary: "Explicitly moving past complexity through character awareness".to_string(),
        }
    }

    fn refine_strategy_from_history(&self, base_strategy: &PivotStrategy, genre: &Genre) -> PivotStrategy {
        // Check our success history with this strategy for this genre
        let genre_key = format!("{:?}", genre);
        
        // If we have successful patterns, lean toward those
        let successful_strategies: Vec<&PivotStrategy> = self.creative_memory.successful_pivots
            .values()
            .filter(|pivot| pivot.success_score > 0.7)
            .map(|pivot| &pivot.strategy_used)
            .collect();

        if successful_strategies.contains(&base_strategy) {
            base_strategy.clone()
        } else if !successful_strategies.is_empty() {
            // Use the most successful strategy we've seen
            successful_strategies[0].clone()
        } else {
            base_strategy.clone()
        }
    }

    fn create_intervention_prompt(
        &self,
        approach: &PivotApproach,
        elements: &[ExtractedElement],
        problematic_content: &str,
        chapter_context: &str
    ) -> Result<String> {
        let mut prompt = String::new();

        // Add context awareness
        prompt.push_str("🔄 NARRATIVE FLOW INTERVENTION\n\n");
        prompt.push_str("The current narrative has become too complex or repetitive for effective continuation. ");
        prompt.push_str(&approach.explanation);
        prompt.push_str("\n\n");

        // Add strategy-specific guidance
        prompt.push_str("CREATIVE DIRECTION:\n");
        prompt.push_str(&approach.creative_direction);
        prompt.push_str("\n\n");

        // Add preservation guidance
        if !approach.preservation_elements.is_empty() {
            prompt.push_str("PRESERVE THESE ELEMENTS:\n");
            for element in &approach.preservation_elements {
                prompt.push_str(&format!("• {}\n", element));
            }
            prompt.push_str("\n");
        }

        // Add abandonment permission
        prompt.push_str("YOU CAN LET GO OF:\n");
        prompt.push_str(&format!("• {}\n", approach.abandonment_summary));
        prompt.push_str("• Complex logical connections that aren't working\n");
        prompt.push_str("• Convoluted plot threads that feel forced\n");
        prompt.push_str("• Any narrative elements that feel stuck or repetitive\n\n");

        // Add creative freedom
        prompt.push_str("CREATIVE FREEDOM:\n");
        prompt.push_str("This is your permission to think outside the box. The goal is engaging, readable content. ");
        prompt.push_str("Trust your creative instincts over logical perfection. ");
        prompt.push_str("Sometimes the best stories come from unexpected turns rather than forced conclusions.\n\n");

        // Add context for the chapter
        prompt.push_str(&format!("CHAPTER CONTEXT: {}\n\n", chapter_context));

        // Add writing instruction
        prompt.push_str("Now write the next section with fresh energy and clarity:");

        Ok(prompt)
    }

    fn explain_intervention_reasoning(&self, flow_decision: &FlowDecision, approach: &PivotApproach) -> String {
        let mut explanation = String::new();
        
        explanation.push_str("Intervention triggered due to: ");
        
        let issue_types: Vec<String> = flow_decision.detected_issues
            .iter()
            .map(|issue| format!("{:?}", issue.indicator_type))
            .collect();
        
        if !issue_types.is_empty() {
            explanation.push_str(&issue_types.join(", "));
        } else {
            explanation.push_str("content complexity patterns");
        }
        
        explanation.push_str(&format!(". Strategy: {:?}. ", approach.strategy));
        explanation.push_str("This will help maintain readability and narrative momentum while preserving valuable story elements.");
        
        explanation
    }

    fn log_intervention(&mut self, approach: &PivotApproach, content: &str, prompt: &str) {
        let intervention = PivotIntervention {
            intervention_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            trigger_reason: approach.explanation.clone(),
            content_before: content.to_string(),
            strategy_applied: approach.strategy.clone(),
            generated_prompt: prompt.to_string(),
            success_rating: None, // Will be set later when we can evaluate
            readability_improvement: None,
            user_satisfaction: None,
        };
        
        self.intervention_history.push(intervention);
    }

    /// Rate the success of a previous intervention
    pub fn rate_intervention_success(&mut self, intervention_id: &str, success_score: f32, readability_improvement: f32) {
        // Find and update the intervention
        let mut intervention_to_learn_from: Option<PivotIntervention> = None;
        
        for intervention in &mut self.intervention_history {
            if intervention.intervention_id == intervention_id {
                intervention.success_rating = Some(success_score);
                intervention.readability_improvement = Some(readability_improvement);
                
                // Clone for learning if successful
                if success_score > 0.7 {
                    intervention_to_learn_from = Some(intervention.clone());
                }
                break;
            }
        }
        
        // Learn from this success/failure outside the borrow
        if let Some(intervention) = intervention_to_learn_from {
            self.learn_from_successful_intervention(&intervention);
        }
    }

    fn learn_from_successful_intervention(&mut self, intervention: &PivotIntervention) {
        // Extract patterns from successful interventions
        let success_pattern = SuccessfulPivot {
            strategy_used: intervention.strategy_applied.clone(),
            context_before: intervention.content_before.clone(),
            context_after: "".to_string(), // Would need the generated content
            extracted_elements: Vec::new(), // Would extract elements
            success_score: intervention.success_rating.unwrap_or(0.0),
            reuse_count: 1,
        };
        
        let pattern_key = format!("{:?}_{}", intervention.strategy_applied, chrono::Utc::now().timestamp());
        self.creative_memory.successful_pivots.insert(pattern_key, success_pattern);
        
        // Update success rate
        let total_interventions = self.intervention_history.len() as f32;
        let successful_interventions = self.intervention_history
            .iter()
            .filter(|i| i.success_rating.unwrap_or(0.0) > 0.7)
            .count() as f32;
        
        self.pivot_success_rate = successful_interventions / total_interventions;
    }

    /// Get statistics about intervention effectiveness
    pub fn get_intervention_stats(&self) -> HashMap<String, f32> {
        let mut stats = HashMap::new();
        
        stats.insert("total_interventions".to_string(), self.intervention_history.len() as f32);
        stats.insert("success_rate".to_string(), self.pivot_success_rate);
        
        // Strategy success rates
        let mut strategy_success: HashMap<String, Vec<f32>> = HashMap::new();
        for intervention in &self.intervention_history {
            if let Some(score) = intervention.success_rating {
                let strategy_key = format!("{:?}", intervention.strategy_applied);
                strategy_success.entry(strategy_key).or_insert_with(Vec::new).push(score);
            }
        }
        
        for (strategy, scores) in strategy_success {
            let avg_score = scores.iter().sum::<f32>() / scores.len() as f32;
            stats.insert(format!("{}_success_rate", strategy), avg_score);
        }
        
        stats
    }

    /// Check if content needs immediate intervention
    pub fn needs_immediate_intervention(&mut self, content: &str) -> Result<bool> {
        let analysis = self.flow_monitor.analyze_content(content, "emergency_check")?;
        
        // Immediate intervention needed if:
        // 1. Very high complexity with very low readability
        // 2. Multiple severe confusion indicators
        // 3. Repetition score is extremely high
        
        Ok(analysis.should_pivot && analysis.confidence > 0.8)
    }

    /// Provide a gentle nudge prompt instead of full intervention
    pub fn create_gentle_nudge(&self, content: &str) -> String {
        format!(
            "The narrative is getting a bit complex. Consider simplifying the current thread or taking the story in a fresh direction. \
            Remember: clarity and reader engagement are more important than resolving every logical detail. \
            Feel free to summarize complex parts and move forward, or shift focus to something more immediate and concrete.\n\n\
            Continue writing with renewed clarity:"
        )
    }

    pub fn get_flow_summary(&self) -> String {
        self.flow_monitor.get_recent_analysis_summary()
    }
}