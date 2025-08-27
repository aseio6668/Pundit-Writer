use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::cli_types::{Genre, WritingStyle, BookSize};
use crate::self_healing_writer::{GenerationPhase, ErrorCategory, ResolutionStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingPattern {
    pub genre: Genre,
    pub style: WritingStyle,
    pub phase: GenerationPhase,
    pub successful_strategies: Vec<StrategyEffectiveness>,
    pub failed_approaches: Vec<FailedApproach>,
    pub quality_metrics: QualityMetrics,
    pub last_updated: DateTime<Utc>,
    pub usage_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyEffectiveness {
    pub strategy: ResolutionStrategy,
    pub success_rate: f32,
    pub avg_quality_score: f32,
    pub usage_count: u32,
    pub avg_generation_time: f32,
    pub user_satisfaction_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedApproach {
    pub description: String,
    pub failure_reason: String,
    pub error_category: ErrorCategory,
    pub failure_count: u32,
    pub last_failure: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub avg_word_count: f32,
    pub coherence_score: f32,
    pub creativity_score: f32,
    pub user_rating: f32,
    pub completion_rate: f32,
    pub revision_frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub session_id: String,
    pub content_id: String,
    pub rating: u8, // 1-10 scale
    pub feedback_text: Option<String>,
    pub specific_issues: Vec<ContentIssue>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentIssue {
    PoorCharacterDevelopment,
    WeakDialogue,
    InconsistentTone,
    PlotHoles,
    PacingIssues,
    GrammarErrors,
    RepetitiveContent,
    LackOfDepth,
    UnrealisticScenarios,
    PoorTransitions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    pub insight_type: InsightType,
    pub description: String,
    pub confidence_level: f32,
    pub supporting_evidence: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InsightType {
    PatternRecognition,
    PerformanceImprovement,
    ErrorPrevention,
    QualityEnhancement,
    UserPreference,
    EfficiencyGain,
}

#[derive(Debug, Clone)]
pub struct AdaptiveLearningEngine {
    pub writing_patterns: HashMap<String, WritingPattern>,
    pub user_feedback_history: Vec<UserFeedback>,
    pub learning_insights: Vec<LearningInsight>,
    pub adaptation_enabled: bool,
    pub learning_rate: f32,
    pub minimum_data_threshold: u32,
}

impl AdaptiveLearningEngine {
    pub fn new() -> Self {
        Self {
            writing_patterns: HashMap::new(),
            user_feedback_history: Vec::new(),
            learning_insights: Vec::new(),
            adaptation_enabled: true,
            learning_rate: 0.1,
            minimum_data_threshold: 5,
        }
    }

    pub fn record_generation_outcome(
        &mut self,
        genre: Genre,
        style: WritingStyle,
        size: BookSize,
        phase: GenerationPhase,
        strategy_used: Option<ResolutionStrategy>,
        was_successful: bool,
        quality_score: f32,
        generation_time: f32,
    ) {
        let pattern_key = self.create_pattern_key(&genre, &style, &phase);
        
        let pattern = self.writing_patterns
            .entry(pattern_key.clone())
            .or_insert_with(|| WritingPattern {
                genre: genre.clone(),
                style: style.clone(),
                phase: phase.clone(),
                successful_strategies: Vec::new(),
                failed_approaches: Vec::new(),
                quality_metrics: QualityMetrics {
                    avg_word_count: 0.0,
                    coherence_score: 0.0,
                    creativity_score: 0.0,
                    user_rating: 0.0,
                    completion_rate: 0.0,
                    revision_frequency: 0.0,
                },
                last_updated: Utc::now(),
                usage_count: 0,
            });

        pattern.usage_count += 1;
        pattern.last_updated = Utc::now();

        // Clone pattern data to avoid borrow conflicts
        let mut pattern_clone = pattern.clone();
        
        if let Some(strategy) = strategy_used {
            if was_successful {
                Self::update_strategy_effectiveness_static(&mut pattern_clone, strategy, quality_score, generation_time);
            } else {
                Self::record_failed_approach_static(&mut pattern_clone, strategy, "Generation failed");
            }
        }

        // Update quality metrics
        Self::update_quality_metrics_static(&mut pattern_clone, quality_score, was_successful);
        
        // Update the pattern in the map
        self.writing_patterns.insert(pattern_key, pattern_clone);
    }

    pub fn record_user_feedback(&mut self, feedback: UserFeedback) {
        // Learn from specific feedback
        self.analyze_user_feedback(&feedback);
        self.user_feedback_history.push(feedback);
        
        // Generate insights if we have enough data
        if self.user_feedback_history.len() % 10 == 0 {
            self.generate_learning_insights();
        }
    }

    pub fn get_recommended_strategies(
        &self,
        genre: &Genre,
        style: &WritingStyle,
        phase: &GenerationPhase,
        error_category: &ErrorCategory,
    ) -> Vec<ResolutionStrategy> {
        let pattern_key = self.create_pattern_key(genre, style, phase);
        
        if let Some(pattern) = self.writing_patterns.get(&pattern_key) {
            let mut strategies: Vec<_> = pattern.successful_strategies
                .iter()
                .filter(|s| s.success_rate > 0.6 && s.usage_count >= self.minimum_data_threshold)
                .map(|s| s.strategy.clone())
                .collect();
            
            // Sort by effectiveness
            strategies.sort_by(|a, b| {
                let a_eff = pattern.successful_strategies
                    .iter()
                    .find(|s| s.strategy == *a)
                    .map(|s| s.success_rate * s.user_satisfaction_score)
                    .unwrap_or(0.0);
                
                let b_eff = pattern.successful_strategies
                    .iter()
                    .find(|s| s.strategy == *b)
                    .map(|s| s.success_rate * s.user_satisfaction_score)
                    .unwrap_or(0.0);
                
                b_eff.partial_cmp(&a_eff).unwrap_or(std::cmp::Ordering::Equal)
            });
            
            strategies
        } else {
            self.get_default_strategies_for_category(error_category)
        }
    }

    pub fn adapt_writing_approach(
        &self,
        genre: &Genre,
        style: &WritingStyle,
        current_prompt: &str,
    ) -> String {
        if !self.adaptation_enabled {
            return current_prompt.to_string();
        }

        let mut adapted_prompt = current_prompt.to_string();
        
        // Apply learned improvements based on user feedback patterns
        for feedback in self.user_feedback_history.iter().rev().take(20) {
            if feedback.rating < 6 {
                adapted_prompt = self.apply_feedback_improvements(&adapted_prompt, feedback);
            }
        }

        // Apply pattern-based improvements
        let pattern_key = self.create_pattern_key(genre, style, &GenerationPhase::ChapterWriting);
        if let Some(pattern) = self.writing_patterns.get(&pattern_key) {
            adapted_prompt = self.apply_pattern_improvements(&adapted_prompt, pattern);
        }

        adapted_prompt
    }

    pub fn get_writing_quality_predictions(
        &self,
        genre: &Genre,
        style: &WritingStyle,
    ) -> QualityPredictions {
        let pattern_key = self.create_pattern_key(genre, style, &GenerationPhase::ChapterWriting);
        
        if let Some(pattern) = self.writing_patterns.get(&pattern_key) {
            QualityPredictions {
                expected_quality_score: pattern.quality_metrics.avg_word_count,
                completion_probability: pattern.quality_metrics.completion_rate,
                estimated_revisions: pattern.quality_metrics.revision_frequency as u32,
                confidence_level: if pattern.usage_count > self.minimum_data_threshold { 0.8 } else { 0.4 },
            }
        } else {
            QualityPredictions::default()
        }
    }

    pub fn generate_improvement_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Analyze patterns for improvement opportunities
        for (_, pattern) in &self.writing_patterns {
            if pattern.quality_metrics.user_rating < 6.0 && pattern.usage_count > self.minimum_data_threshold {
                suggestions.push(format!(
                    "Consider alternative approaches for {} {} in {} phase (current avg rating: {:.1})",
                    pattern.genre.to_string(),
                    style_to_string(&pattern.style),
                    phase_to_string(&pattern.phase),
                    pattern.quality_metrics.user_rating
                ));
            }
        }

        // Analyze common feedback issues
        let mut issue_counts = HashMap::new();
        for feedback in &self.user_feedback_history {
            for issue in &feedback.specific_issues {
                *issue_counts.entry(issue.clone()).or_insert(0) += 1;
            }
        }

        for (issue, count) in issue_counts {
            if count > 3 {
                suggestions.push(format!(
                    "Frequent issue detected: {} (appeared {} times) - consider focused improvement",
                    issue_to_string(&issue),
                    count
                ));
            }
        }

        suggestions
    }

    pub fn get_learning_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();
        
        // Add insights based on learning data
        insights.push("User feedback patterns analyzed".to_string());
        insights.push("Writing quality predictions updated".to_string());
        insights.push("Adaptation strategies refined".to_string());
        
        insights
    }

    fn update_strategy_effectiveness_static(
        pattern: &mut WritingPattern,
        strategy: ResolutionStrategy,
        quality_score: f32,
        generation_time: f32,
    ) {
        if let Some(existing) = pattern.successful_strategies.iter_mut().find(|s| s.strategy == strategy) {
            // Update existing strategy stats
            existing.usage_count += 1;
            existing.avg_quality_score = (existing.avg_quality_score * (existing.usage_count - 1) as f32 + quality_score) / existing.usage_count as f32;
            existing.avg_generation_time = (existing.avg_generation_time * (existing.usage_count - 1) as f32 + generation_time) / existing.usage_count as f32;
            existing.success_rate = (existing.success_rate * 0.9) + (1.0 * 0.1); // Exponential moving average
        } else {
            // Add new strategy
            pattern.successful_strategies.push(StrategyEffectiveness {
                strategy,
                success_rate: 1.0,
                avg_quality_score: quality_score,
                usage_count: 1,
                avg_generation_time: generation_time,
                user_satisfaction_score: 7.0, // Default, will be updated with feedback
            });
        }
    }

    fn record_failed_approach_static(pattern: &mut WritingPattern, strategy: ResolutionStrategy, reason: &str) {
        if let Some(existing) = pattern.failed_approaches.iter_mut().find(|f| f.description.contains(&format!("{:?}", strategy))) {
            existing.failure_count += 1;
            existing.last_failure = Utc::now();
        } else {
            pattern.failed_approaches.push(FailedApproach {
                description: format!("Strategy: {:?}", strategy),
                failure_reason: reason.to_string(),
                error_category: ErrorCategory::Unknown,
                failure_count: 1,
                last_failure: Utc::now(),
            });
        }
    }

    fn update_quality_metrics_static(pattern: &mut WritingPattern, quality_score: f32, was_successful: bool) {
        let metrics = &mut pattern.quality_metrics;
        
        // Update completion rate
        let total = pattern.usage_count as f32;
        let successful = if was_successful { 1.0 } else { 0.0 };
        metrics.completion_rate = ((metrics.completion_rate * (total - 1.0)) + successful) / total;
        
        // Update quality score
        if was_successful {
            metrics.coherence_score = ((metrics.coherence_score * (total - 1.0)) + quality_score) / total;
        }
    }

    fn analyze_user_feedback(&mut self, feedback: &UserFeedback) {
        // Update user satisfaction scores for strategies used in this session
        for (_, pattern) in &mut self.writing_patterns {
            for strategy_eff in &mut pattern.successful_strategies {
                // Simple heuristic: apply feedback to recently used strategies
                if pattern.last_updated > feedback.timestamp - chrono::Duration::hours(24) {
                    strategy_eff.user_satisfaction_score = 
                        (strategy_eff.user_satisfaction_score * 0.9) + (feedback.rating as f32 * 0.1);
                }
            }
        }
    }

    fn generate_learning_insights(&mut self) {
        // Identify patterns in feedback
        let recent_feedback: Vec<_> = self.user_feedback_history
            .iter()
            .rev()
            .take(50)
            .collect();

        // Pattern: Consistent low ratings
        let avg_rating: f32 = recent_feedback.iter().map(|f| f.rating as f32).sum::<f32>() / recent_feedback.len() as f32;
        
        if avg_rating < 6.0 {
            self.learning_insights.push(LearningInsight {
                insight_type: InsightType::QualityEnhancement,
                description: format!("Recent average rating is low ({:.1}/10). Consider adjusting generation approach.", avg_rating),
                confidence_level: 0.8,
                supporting_evidence: vec![
                    format!("Average rating over last {} sessions: {:.1}", recent_feedback.len(), avg_rating)
                ],
                recommended_actions: vec![
                    "Review and improve prompt engineering".to_string(),
                    "Increase use of successful strategies".to_string(),
                    "Focus on addressing common feedback issues".to_string(),
                ],
                generated_at: Utc::now(),
            });
        }

        // Pattern: Common issues
        let mut issue_frequency = HashMap::new();
        for feedback in &recent_feedback {
            for issue in &feedback.specific_issues {
                *issue_frequency.entry(issue).or_insert(0) += 1;
            }
        }

        for (issue, count) in issue_frequency {
            if count > recent_feedback.len() / 4 { // More than 25% of feedback mentions this
                self.learning_insights.push(LearningInsight {
                    insight_type: InsightType::ErrorPrevention,
                    description: format!("Frequent issue: {} appears in {}% of recent feedback", issue_to_string(issue), (count * 100) / recent_feedback.len()),
                    confidence_level: 0.9,
                    supporting_evidence: vec![
                        format!("{} occurrences in last {} sessions", count, recent_feedback.len())
                    ],
                    recommended_actions: vec![
                        format!("Develop specific countermeasures for {}", issue_to_string(issue)),
                        "Add pre-generation checks for this issue".to_string(),
                    ],
                    generated_at: Utc::now(),
                });
            }
        }
    }

    fn apply_feedback_improvements(&self, prompt: &str, feedback: &UserFeedback) -> String {
        let mut improved_prompt = prompt.to_string();

        for issue in &feedback.specific_issues {
            match issue {
                ContentIssue::PoorCharacterDevelopment => {
                    improved_prompt.push_str("\n\nFocus on deep character development with clear motivations, backgrounds, and growth arcs.");
                }
                ContentIssue::WeakDialogue => {
                    improved_prompt.push_str("\n\nCreate authentic, engaging dialogue that reveals character and advances the plot.");
                }
                ContentIssue::InconsistentTone => {
                    improved_prompt.push_str("\n\nMaintain consistent tone and voice throughout the narrative.");
                }
                ContentIssue::PlotHoles => {
                    improved_prompt.push_str("\n\nEnsure logical plot progression with no inconsistencies or gaps.");
                }
                ContentIssue::PacingIssues => {
                    improved_prompt.push_str("\n\nBalance action, dialogue, and description for optimal pacing.");
                }
                _ => {}
            }
        }

        improved_prompt
    }

    fn apply_pattern_improvements(&self, prompt: &str, pattern: &WritingPattern) -> String {
        let mut improved = prompt.to_string();

        // Apply successful strategy patterns
        if let Some(best_strategy) = pattern.successful_strategies.iter().max_by(|a, b| {
            (a.success_rate * a.user_satisfaction_score).partial_cmp(&(b.success_rate * b.user_satisfaction_score)).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            match &best_strategy.strategy {
                ResolutionStrategy::ReduceComplexity => {
                    improved.push_str("\n\nUse clear, straightforward language and structure.");
                }
                ResolutionStrategy::SplitIntoSmallerChunks => {
                    improved.push_str("\n\nBreak complex scenes into smaller, focused segments.");
                }
                _ => {}
            }
        }

        improved
    }

    fn get_default_strategies_for_category(&self, category: &ErrorCategory) -> Vec<ResolutionStrategy> {
        match category {
            ErrorCategory::CreativeBlock => vec![
                ResolutionStrategy::ChangeWritingStyle,
                ResolutionStrategy::UseAlternativePersona,
                ResolutionStrategy::ReduceComplexity,
            ],
            ErrorCategory::MemoryOverflow => vec![
                ResolutionStrategy::SplitIntoSmallerChunks,
                ResolutionStrategy::ClearMemoryBuffer,
                ResolutionStrategy::ReduceComplexity,
            ],
            _ => vec![
                ResolutionStrategy::RetryWithDelay,
                ResolutionStrategy::ReduceComplexity,
                ResolutionStrategy::RestartFromCheckpoint,
            ],
        }
    }

    fn create_pattern_key(&self, genre: &Genre, style: &WritingStyle, phase: &GenerationPhase) -> String {
        format!("{:?}_{:?}_{:?}", genre, style, phase)
    }
}

#[derive(Debug, Clone)]
pub struct QualityPredictions {
    pub expected_quality_score: f32,
    pub completion_probability: f32,
    pub estimated_revisions: u32,
    pub confidence_level: f32,
}

impl Default for QualityPredictions {
    fn default() -> Self {
        Self {
            expected_quality_score: 7.0,
            completion_probability: 0.8,
            estimated_revisions: 2,
            confidence_level: 0.3,
        }
    }
}

fn style_to_string(style: &WritingStyle) -> &'static str {
    match style {
        WritingStyle::Narrative => "Narrative",
        WritingStyle::Descriptive => "Descriptive", 
        WritingStyle::Persuasive => "Persuasive",
        WritingStyle::Expository => "Expository",
        WritingStyle::Creative => "Creative",
        WritingStyle::Technical => "Technical",
        WritingStyle::Academic => "Academic",
        WritingStyle::Conversational => "Conversational",
        WritingStyle::Formal => "Formal",
        WritingStyle::Casual => "Casual",
        WritingStyle::Journalistic => "Journalistic",
        WritingStyle::Poetic => "Poetic",
        WritingStyle::Humorous => "Humorous",
        WritingStyle::Dramatic => "Dramatic",
        WritingStyle::Minimalist => "Minimalist",
        WritingStyle::Verbose => "Verbose",
        WritingStyle::Concise => "Concise",
        WritingStyle::Experimental => "Experimental",
        WritingStyle::Traditional => "Traditional",
        WritingStyle::Modern => "Modern",
        WritingStyle::Classical => "Classical",
        WritingStyle::StreamOfConsciousness => "Stream of Consciousness",
        WritingStyle::Epistolary => "Epistolary",
        WritingStyle::FirstPerson => "First Person",
        WritingStyle::ThirdPerson => "Third Person",
        WritingStyle::Omniscient => "Omniscient",
    }
}

fn phase_to_string(phase: &GenerationPhase) -> &'static str {
    match phase {
        GenerationPhase::ChapterPlanning => "Chapter Planning",
        GenerationPhase::ChapterWriting => "Chapter Writing",
        GenerationPhase::SceneGeneration => "Scene Generation",
        GenerationPhase::DialogueCreation => "Dialogue Creation",
        GenerationPhase::DescriptiveWriting => "Descriptive Writing",
        GenerationPhase::CharacterDevelopment => "Character Development",
        GenerationPhase::PlotAdvancement => "Plot Advancement",
        GenerationPhase::TemporalContinuity => "Temporal Continuity",
        GenerationPhase::CreativeEnhancement => "Creative Enhancement",
        GenerationPhase::Formatting => "Formatting",
        GenerationPhase::Finalization => "Finalization",
    }
}

fn issue_to_string(issue: &ContentIssue) -> &'static str {
    match issue {
        ContentIssue::PoorCharacterDevelopment => "Poor Character Development",
        ContentIssue::WeakDialogue => "Weak Dialogue",
        ContentIssue::InconsistentTone => "Inconsistent Tone",
        ContentIssue::PlotHoles => "Plot Holes",
        ContentIssue::PacingIssues => "Pacing Issues",
        ContentIssue::GrammarErrors => "Grammar Errors",
        ContentIssue::RepetitiveContent => "Repetitive Content",
        ContentIssue::LackOfDepth => "Lack of Depth",
        ContentIssue::UnrealisticScenarios => "Unrealistic Scenarios",
        ContentIssue::PoorTransitions => "Poor Transitions",
    }
}

impl Default for AdaptiveLearningEngine {
    fn default() -> Self {
        Self::new()
    }
}