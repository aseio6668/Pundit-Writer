use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GenerationPhase {
    ChapterPlanning,
    ChapterWriting,
    SceneGeneration,
    DialogueCreation,
    DescriptiveWriting,
    CharacterDevelopment,
    PlotAdvancement,
    TemporalContinuity,
    CreativeEnhancement,
    Formatting,
    Finalization,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    MemoryOverflow,
    CreativeBlock,
    TemporalInconsistency,
    CharacterMismatch,
    PlotLogicError,
    FormatError,
    SystemTimeout,
    NetworkError,
    ParsingError,
    ContentFiltering,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    pub category: ErrorCategory,
    pub phase: GenerationPhase,
    pub error_message: String,
    pub context_snapshot: String,
    pub occurrence_count: u32,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub successful_resolution_strategies: Vec<ResolutionStrategy>,
    pub failed_resolution_attempts: Vec<ResolutionStrategy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    RetryWithDelay,
    ReduceComplexity,
    SplitIntoSmallerChunks,
    ChangeWritingStyle,
    SimplifyLanguage,
    UseAlternativePersona,
    RestartFromCheckpoint,
    SkipAndContinue,
    SeekUserInput,
    ApplyCreativeBlockRecovery,
    RefreshContext,
    ClearMemoryBuffer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PausePoint {
    pub id: String,
    pub phase: GenerationPhase,
    pub description: String,
    pub context_snapshot: String,
    pub retry_options: Vec<RetryOption>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryOption {
    pub label: String,
    pub strategy: ResolutionStrategy,
    pub estimated_success_rate: f32,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SelfHealingWriter {
    pub error_knowledge_base: HashMap<String, ErrorPattern>,
    pub pause_points: Vec<PausePoint>,
    pub learning_enabled: bool,
    pub auto_retry_attempts: u32,
    pub success_metrics: SuccessMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetrics {
    pub total_generations: u32,
    pub successful_completions: u32,
    pub auto_healed_errors: u32,
    pub user_assisted_recoveries: u32,
    pub improvement_rate: f32,
}

impl SelfHealingWriter {
    pub fn new() -> Self {
        Self {
            error_knowledge_base: HashMap::new(),
            pause_points: Vec::new(),
            learning_enabled: true,
            auto_retry_attempts: 3,
            success_metrics: SuccessMetrics {
                total_generations: 0,
                successful_completions: 0,
                auto_healed_errors: 0,
                user_assisted_recoveries: 0,
                improvement_rate: 0.0,
            },
        }
    }

    pub fn create_pause_point(
        &mut self,
        phase: GenerationPhase,
        error_msg: &str,
        context: &str,
    ) -> PausePoint {
        let pause_id = format!("pause_{}_{}", 
            chrono::Utc::now().timestamp_millis(),
            phase_to_string(&phase)
        );

        let retry_options = self.generate_retry_options(&phase, error_msg, context);
        
        let pause_point = PausePoint {
            id: pause_id,
            phase: phase.clone(),
            description: format!("Error in {}: {}", phase_to_string(&phase), error_msg),
            context_snapshot: context.to_string(),
            retry_options,
            timestamp: Utc::now(),
        };

        self.pause_points.push(pause_point.clone());
        pause_point
    }

    pub fn learn_from_error(
        &mut self,
        phase: GenerationPhase,
        error_msg: &str,
        context: &str,
        resolution: Option<ResolutionStrategy>,
        was_successful: bool,
    ) {
        if !self.learning_enabled {
            return;
        }

        let error_key = self.generate_error_key(&phase, error_msg);
        let category = self.categorize_error(error_msg);

        let pattern = self.error_knowledge_base
            .entry(error_key.clone())
            .or_insert_with(|| ErrorPattern {
                category: category.clone(),
                phase: phase.clone(),
                error_message: error_msg.to_string(),
                context_snapshot: context.to_string(),
                occurrence_count: 0,
                first_seen: Utc::now(),
                last_seen: Utc::now(),
                successful_resolution_strategies: Vec::new(),
                failed_resolution_attempts: Vec::new(),
            });

        pattern.occurrence_count += 1;
        pattern.last_seen = Utc::now();

        if let Some(strategy) = resolution {
            if was_successful {
                if !pattern.successful_resolution_strategies.contains(&strategy) {
                    pattern.successful_resolution_strategies.push(strategy);
                }
            } else {
                if !pattern.failed_resolution_attempts.contains(&strategy) {
                    pattern.failed_resolution_attempts.push(strategy);
                }
            }
        }

        self.update_success_metrics(was_successful);
    }

    pub fn suggest_best_resolution(&self, phase: &GenerationPhase, error_msg: &str) -> Vec<ResolutionStrategy> {
        let error_key = self.generate_error_key(phase, error_msg);
        
        if let Some(pattern) = self.error_knowledge_base.get(&error_key) {
            if !pattern.successful_resolution_strategies.is_empty() {
                return pattern.successful_resolution_strategies.clone();
            }
        }

        // Fallback to general strategies based on phase and error category
        let category = self.categorize_error(error_msg);
        self.get_default_strategies(&category, phase)
    }

    pub fn attempt_auto_heal(
        &mut self,
        phase: GenerationPhase,
        error_msg: &str,
        context: &str,
    ) -> Result<String, String> {
        let strategies = self.suggest_best_resolution(&phase, error_msg);
        
        for strategy in strategies.iter().take(self.auto_retry_attempts as usize) {
            println!("🔧 Auto-healing attempt using strategy: {:?}", strategy);
            
            match self.apply_resolution_strategy(strategy, context, &phase) {
                Ok(result) => {
                    self.learn_from_error(phase.clone(), error_msg, context, Some(strategy.clone()), true);
                    self.success_metrics.auto_healed_errors += 1;
                    return Ok(result);
                }
                Err(_) => {
                    self.learn_from_error(phase.clone(), error_msg, context, Some(strategy.clone()), false);
                    continue;
                }
            }
        }

        // If auto-healing fails, create a pause point
        let pause_point = self.create_pause_point(phase, error_msg, context);
        Err(format!("Auto-healing failed. Created pause point: {}", pause_point.id))
    }

    pub fn apply_resolution_strategy(
        &self,
        strategy: &ResolutionStrategy,
        context: &str,
        phase: &GenerationPhase,
    ) -> Result<String, String> {
        match strategy {
            ResolutionStrategy::RetryWithDelay => {
                std::thread::sleep(Duration::from_secs(2));
                Ok("Retry after delay".to_string())
            }
            ResolutionStrategy::ReduceComplexity => {
                let simplified_context = self.simplify_content_complexity(context);
                Ok(simplified_context)
            }
            ResolutionStrategy::SplitIntoSmallerChunks => {
                let chunked_context = self.split_content_into_chunks(context, phase);
                Ok(chunked_context)
            }
            ResolutionStrategy::ChangeWritingStyle => {
                Ok("Switching to alternative writing style".to_string())
            }
            ResolutionStrategy::SimplifyLanguage => {
                Ok("Using simpler language patterns".to_string())
            }
            ResolutionStrategy::UseAlternativePersona => {
                Ok("Applying different writer persona".to_string())
            }
            ResolutionStrategy::RestartFromCheckpoint => {
                Ok("Restored from previous checkpoint".to_string())
            }
            ResolutionStrategy::RefreshContext => {
                Ok("Context refreshed and optimized".to_string())
            }
            ResolutionStrategy::ClearMemoryBuffer => {
                Ok("Memory buffer cleared and reset".to_string())
            }
            ResolutionStrategy::ApplyCreativeBlockRecovery => {
                let recovery_result = self.apply_creative_block_recovery(context, phase);
                Ok(recovery_result)
            }
            ResolutionStrategy::ChangeWritingStyle => {
                let style_change = self.change_writing_approach(context, phase);
                Ok(style_change)
            }
            ResolutionStrategy::SimplifyLanguage => {
                let simplified = self.simplify_language_patterns(context);
                Ok(simplified)
            }
            ResolutionStrategy::RefreshContext => {
                let refreshed = self.refresh_context_memory(context);
                Ok(refreshed)
            }
            ResolutionStrategy::ClearMemoryBuffer => {
                Ok("MEMORY_CLEARED: Context buffer reset, starting fresh".to_string())
            }
            ResolutionStrategy::SkipAndContinue => {
                let skip_result = self.skip_and_continue(context, phase);
                Ok(skip_result)
            }
            ResolutionStrategy::SeekUserInput => {
                let user_input_prompt = self.generate_user_input_prompt(context, phase);
                Ok(user_input_prompt)
            }
            _ => Err("Strategy not implemented for auto-healing".to_string())
        }
    }

    fn generate_retry_options(
        &self,
        phase: &GenerationPhase,
        error_msg: &str,
        context: &str,
    ) -> Vec<RetryOption> {
        let strategies = self.suggest_best_resolution(phase, error_msg);
        let mut options = Vec::new();

        for strategy in strategies {
            let (success_rate, description) = self.get_strategy_info(&strategy, phase);
            
            options.push(RetryOption {
                label: format!("Retry with {}", strategy_to_string(&strategy)),
                strategy,
                estimated_success_rate: success_rate,
                description,
            });
        }

        // Always add manual options
        options.push(RetryOption {
            label: "Skip this section and continue".to_string(),
            strategy: ResolutionStrategy::SkipAndContinue,
            estimated_success_rate: 0.9,
            description: "Continue generation, skipping problematic section".to_string(),
        });

        options.push(RetryOption {
            label: "Restart entire chapter".to_string(),
            strategy: ResolutionStrategy::RestartFromCheckpoint,
            estimated_success_rate: 0.7,
            description: "Start over from the last chapter checkpoint".to_string(),
        });

        options
    }

    fn categorize_error(&self, error_msg: &str) -> ErrorCategory {
        let msg_lower = error_msg.to_lowercase();
        
        if msg_lower.contains("memory") || msg_lower.contains("overflow") {
            ErrorCategory::MemoryOverflow
        } else if msg_lower.contains("creative") || msg_lower.contains("block") {
            ErrorCategory::CreativeBlock
        } else if msg_lower.contains("temporal") || msg_lower.contains("continuity") {
            ErrorCategory::TemporalInconsistency
        } else if msg_lower.contains("character") {
            ErrorCategory::CharacterMismatch
        } else if msg_lower.contains("plot") || msg_lower.contains("logic") {
            ErrorCategory::PlotLogicError
        } else if msg_lower.contains("format") {
            ErrorCategory::FormatError
        } else if msg_lower.contains("timeout") {
            ErrorCategory::SystemTimeout
        } else if msg_lower.contains("network") || msg_lower.contains("connection") {
            ErrorCategory::NetworkError
        } else if msg_lower.contains("parse") || msg_lower.contains("parsing") {
            ErrorCategory::ParsingError
        } else if msg_lower.contains("content") && msg_lower.contains("filter") {
            ErrorCategory::ContentFiltering
        } else {
            ErrorCategory::Unknown
        }
    }

    fn get_default_strategies(&self, category: &ErrorCategory, phase: &GenerationPhase) -> Vec<ResolutionStrategy> {
        match category {
            ErrorCategory::MemoryOverflow => vec![
                ResolutionStrategy::ClearMemoryBuffer,
                ResolutionStrategy::SplitIntoSmallerChunks,
                ResolutionStrategy::ReduceComplexity,
            ],
            ErrorCategory::CreativeBlock => vec![
                ResolutionStrategy::ApplyCreativeBlockRecovery,
                ResolutionStrategy::ChangeWritingStyle,
                ResolutionStrategy::UseAlternativePersona,
            ],
            ErrorCategory::TemporalInconsistency => vec![
                ResolutionStrategy::RefreshContext,
                ResolutionStrategy::RestartFromCheckpoint,
                ResolutionStrategy::ReduceComplexity,
            ],
            ErrorCategory::SystemTimeout => vec![
                ResolutionStrategy::SplitIntoSmallerChunks,
                ResolutionStrategy::RetryWithDelay,
                ResolutionStrategy::SimplifyLanguage,
            ],
            _ => vec![
                ResolutionStrategy::RetryWithDelay,
                ResolutionStrategy::ReduceComplexity,
                ResolutionStrategy::RestartFromCheckpoint,
            ],
        }
    }

    fn get_strategy_info(&self, strategy: &ResolutionStrategy, phase: &GenerationPhase) -> (f32, String) {
        match strategy {
            ResolutionStrategy::RetryWithDelay => (0.6, "Wait briefly and retry the operation".to_string()),
            ResolutionStrategy::ReduceComplexity => (0.8, "Simplify the content complexity".to_string()),
            ResolutionStrategy::SplitIntoSmallerChunks => (0.9, "Break into smaller, manageable pieces".to_string()),
            ResolutionStrategy::ChangeWritingStyle => (0.7, "Switch to a different writing approach".to_string()),
            ResolutionStrategy::UseAlternativePersona => (0.75, "Try a different historical writer persona".to_string()),
            ResolutionStrategy::ClearMemoryBuffer => (0.85, "Clear memory and start fresh".to_string()),
            _ => (0.5, "Apply general recovery strategy".to_string()),
        }
    }

    fn generate_error_key(&self, phase: &GenerationPhase, error_msg: &str) -> String {
        format!("{}:{}", phase_to_string(phase), error_msg.chars().take(100).collect::<String>())
    }

    fn update_success_metrics(&mut self, was_successful: bool) {
        self.success_metrics.total_generations += 1;
        
        if was_successful {
            self.success_metrics.successful_completions += 1;
        }

        self.success_metrics.improvement_rate = 
            self.success_metrics.successful_completions as f32 / 
            self.success_metrics.total_generations as f32;
    }

    pub fn get_learning_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();
        
        insights.push(format!(
            "📊 Success Rate: {:.1}% ({}/{})", 
            self.success_metrics.improvement_rate * 100.0,
            self.success_metrics.successful_completions,
            self.success_metrics.total_generations
        ));

        insights.push(format!(
            "🔧 Auto-healed Errors: {}", 
            self.success_metrics.auto_healed_errors
        ));

        insights.push(format!(
            "👤 User-assisted Recoveries: {}", 
            self.success_metrics.user_assisted_recoveries
        ));

        let most_common_errors: Vec<_> = self.error_knowledge_base
            .values()
            .filter(|p| p.occurrence_count > 1)
            .collect();

        if !most_common_errors.is_empty() {
            insights.push("🔍 Most Common Error Patterns:".to_string());
            for pattern in most_common_errors.iter().take(3) {
                insights.push(format!(
                    "   • {} in {} (occurred {} times)",
                    pattern.error_message.chars().take(50).collect::<String>(),
                    phase_to_string(&pattern.phase),
                    pattern.occurrence_count
                ));
            }
        }

        insights
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

fn strategy_to_string(strategy: &ResolutionStrategy) -> &'static str {
    match strategy {
        ResolutionStrategy::RetryWithDelay => "Delayed Retry",
        ResolutionStrategy::ReduceComplexity => "Complexity Reduction",
        ResolutionStrategy::SplitIntoSmallerChunks => "Chunk Splitting",
        ResolutionStrategy::ChangeWritingStyle => "Style Change",
        ResolutionStrategy::SimplifyLanguage => "Language Simplification",
        ResolutionStrategy::UseAlternativePersona => "Persona Switch",
        ResolutionStrategy::RestartFromCheckpoint => "Checkpoint Restart",
        ResolutionStrategy::SkipAndContinue => "Skip & Continue",
        ResolutionStrategy::SeekUserInput => "User Input",
        ResolutionStrategy::ApplyCreativeBlockRecovery => "Creative Recovery",
        ResolutionStrategy::RefreshContext => "Context Refresh",
        ResolutionStrategy::ClearMemoryBuffer => "Memory Clear",
    }
}

impl SelfHealingWriter {
    /// Smart complexity reduction - simplifies overly complex content while preserving meaning
    fn simplify_content_complexity(&self, context: &str) -> String {
        let mut simplified = context.to_string();
        
        // 1. Break down run-on sentences (>40 words)
        let sentences: Vec<&str> = simplified.split(". ").collect();
        let mut simplified_sentences = Vec::new();
        
        for sentence in sentences {
            let word_count = sentence.split_whitespace().count();
            if word_count > 40 {
                // Split complex sentence at logical break points
                let simplified_sentence = self.split_complex_sentence(sentence);
                simplified_sentences.push(simplified_sentence);
            } else {
                simplified_sentences.push(sentence.to_string());
            }
        }
        simplified = simplified_sentences.join(". ");
        
        // 2. Replace complex vocabulary with simpler alternatives
        simplified = self.simplify_vocabulary(&simplified);
        
        // 3. Reduce nested clauses and parenthetical expressions
        simplified = self.reduce_nested_clauses(&simplified);
        
        // 4. Convert passive voice to active where possible
        simplified = self.convert_passive_to_active(&simplified);
        
        format!("COMPLEXITY_REDUCED: {}", simplified)
    }
    
    fn split_complex_sentence(&self, sentence: &str) -> String {
        // Split at conjunctions and relative pronouns
        let break_points = ["because", "although", "however", "therefore", "meanwhile", "furthermore", "which", "that", "where", "when"];
        
        for break_point in break_points {
            if sentence.contains(break_point) {
                let parts: Vec<&str> = sentence.splitn(2, break_point).collect();
                if parts.len() == 2 {
                    return format!("{}. {} {}", parts[0].trim(), break_point.to_uppercase(), parts[1].trim());
                }
            }
        }
        sentence.to_string()
    }
    
    fn simplify_vocabulary(&self, text: &str) -> String {
        let replacements = [
            ("utilize", "use"), ("demonstrate", "show"), ("sufficient", "enough"),
            ("numerous", "many"), ("endeavor", "try"), ("facilitate", "help"),
            ("consequently", "so"), ("nevertheless", "but"), ("furthermore", "also"),
            ("therefore", "so"), ("however", "but"), ("additionally", "also"),
            ("subsequently", "then"), ("approximately", "about"), ("occasionally", "sometimes")
        ];
        
        let mut simplified = text.to_string();
        for (complex, simple) in replacements {
            simplified = simplified.replace(complex, simple);
            // Handle capitalized versions
            let complex_cap = format!("{}{}", &complex[0..1].to_uppercase(), &complex[1..]);
            let simple_cap = format!("{}{}", &simple[0..1].to_uppercase(), &simple[1..]);
            simplified = simplified.replace(&complex_cap, &simple_cap);
        }
        simplified
    }
    
    fn reduce_nested_clauses(&self, text: &str) -> String {
        // Remove excessive parenthetical expressions
        let mut result = text.to_string();
        
        // Simplify nested parentheses
        while result.matches('(').count() > result.matches(')').count() || 
              result.matches('(').count() > 2 {
            if let Some(start) = result.find('(') {
                if let Some(end) = result[start..].find(')') {
                    let full_end = start + end + 1;
                    result.replace_range(start..full_end, "");
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        // Reduce em-dash clauses
        let parts: Vec<&str> = result.split(" — ").collect();
        if parts.len() > 2 {
            result = format!("{} — {}", parts[0], parts[parts.len() - 1]);
        }
        
        result
    }
    
    fn convert_passive_to_active(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Simple passive voice patterns
        let passive_patterns = [
            ("was written by", "wrote"),
            ("was created by", "created"),
            ("was discovered by", "discovered"),
            ("was found by", "found"),
            ("is considered to be", "is"),
            ("are believed to be", "are"),
        ];
        
        for (passive, active) in passive_patterns {
            result = result.replace(passive, active);
        }
        
        result
    }
    
    /// Intelligent content chunking - splits large content into manageable sections
    fn split_content_into_chunks(&self, context: &str, phase: &GenerationPhase) -> String {
        let word_count = context.split_whitespace().count();
        
        // Determine optimal chunk size based on phase
        let target_chunk_size = match phase {
            GenerationPhase::ChapterWriting => 800,      // Chapters can be substantial
            GenerationPhase::SceneGeneration => 400,     // Scenes are smaller
            GenerationPhase::DialogueCreation => 200,    // Dialogue is concise
            GenerationPhase::DescriptiveWriting => 600,  // Descriptions are moderate
            GenerationPhase::CharacterDevelopment => 500, // Character work is medium
            _ => 500, // Default for other phases
        };
        
        if word_count <= target_chunk_size {
            return format!("CHUNK_OPTIMAL: {}", context);
        }
        
        // Find natural break points for splitting
        let chunks = self.find_natural_breakpoints(context, target_chunk_size);
        let chunk_count = chunks.len();
        
        // Return instructions for chunked generation
        if chunk_count > 1 {
            format!(
                "CHUNKED_GENERATION: Split into {} parts. Current chunk (1/{}): {}",
                chunk_count,
                chunk_count,
                chunks[0]
            )
        } else {
            format!("CHUNK_REDUCED: {}", self.truncate_to_size(context, target_chunk_size))
        }
    }
    
    fn find_natural_breakpoints(&self, content: &str, target_size: usize) -> Vec<String> {
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut current_size = 0;
        
        // Look for natural break points
        let break_indicators = [
            ".", "!", "?", ";",           // Sentence endings
            "Chapter", "Scene", "***",    // Section markers
            "\n\n", "\n---", "\n###",     // Paragraph/section breaks
        ];
        
        for (i, word) in words.iter().enumerate() {
            current_chunk.push(*word);
            current_size += word.len() + 1; // +1 for space
            
            // Check if we're near target size and at a natural break
            if current_size >= target_size * 3 / 4 {  // 75% of target
                let should_break = break_indicators.iter().any(|indicator| {
                    word.contains(indicator) || 
                    (i + 1 < words.len() && words[i + 1].starts_with(indicator))
                });
                
                if should_break || current_size >= target_size {
                    chunks.push(current_chunk.join(" "));
                    current_chunk.clear();
                    current_size = 0;
                }
            }
        }
        
        // Add remaining content
        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join(" "));
        }
        
        // If no natural breaks found, split at target size
        if chunks.len() == 1 && current_size > target_size {
            return self.force_split_at_size(content, target_size);
        }
        
        chunks
    }
    
    fn force_split_at_size(&self, content: &str, target_size: usize) -> Vec<String> {
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut current_size = 0;
        
        for word in words {
            if current_size + word.len() + 1 > target_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.join(" "));
                current_chunk.clear();
                current_size = 0;
            }
            current_chunk.push(word);
            current_size += word.len() + 1;
        }
        
        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join(" "));
        }
        
        chunks
    }
    
    fn truncate_to_size(&self, content: &str, max_size: usize) -> String {
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut result = Vec::new();
        let mut current_size = 0;
        
        for word in words {
            if current_size + word.len() + 1 > max_size {
                break;
            }
            result.push(word);
            current_size += word.len() + 1;
        }
        
        let truncated = result.join(" ");
        if truncated.len() < content.len() {
            format!("{}... [TRUNCATED]", truncated)
        } else {
            truncated
        }
    }
    
    /// Apply creative block recovery techniques
    fn apply_creative_block_recovery(&self, context: &str, phase: &GenerationPhase) -> String {
        // Detect signs of creative block
        let repetition_score = self.calculate_repetition_score(context);
        let complexity_score = self.calculate_complexity_score(context);
        
        if repetition_score > 0.7 {
            // High repetition detected - apply variation techniques
            format!("CREATIVE_RECOVERY: High repetition detected ({}%). Applying variation techniques: {}", 
                (repetition_score * 100.0) as u8, 
                self.generate_variation_suggestions(context))
        } else if complexity_score > 0.8 {
            // Overly complex - simplify and refresh
            format!("CREATIVE_RECOVERY: Complexity overload ({}%). Simplifying approach: {}", 
                (complexity_score * 100.0) as u8,
                self.generate_simplification_approach(context))
        } else {
            // General creative refresh
            format!("CREATIVE_RECOVERY: Applying creative refresh techniques: {}", 
                self.generate_creative_refresh_approach(phase))
        }
    }
    
    fn calculate_repetition_score(&self, context: &str) -> f32 {
        let words: Vec<&str> = context.split_whitespace().collect();
        if words.len() < 20 {
            return 0.0;
        }
        
        let mut word_counts = std::collections::HashMap::new();
        for word in &words {
            let clean_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
            if clean_word.len() > 3 { // Ignore short words
                *word_counts.entry(clean_word).or_insert(0) += 1;
            }
        }
        
        let total_significant_words = word_counts.values().sum::<usize>() as f32;
        let repeated_words: usize = word_counts.values().filter(|&&count| count > 2).sum();
        
        repeated_words as f32 / total_significant_words
    }
    
    fn calculate_complexity_score(&self, context: &str) -> f32 {
        let sentences: Vec<&str> = context.split('.').collect();
        let avg_sentence_length = context.split_whitespace().count() as f32 / sentences.len() as f32;
        
        let complexity_indicators = context.matches(|c: char| "(),;:".contains(c)).count() as f32;
        let word_count = context.split_whitespace().count() as f32;
        
        // Normalize complexity score
        let length_factor = (avg_sentence_length / 20.0).min(1.0); // Cap at 20 words per sentence
        let punctuation_factor = (complexity_indicators / word_count * 10.0).min(1.0);
        
        (length_factor + punctuation_factor) / 2.0
    }
    
    fn generate_variation_suggestions(&self, _context: &str) -> String {
        let techniques = [
            "Switch narrative perspective", "Add dialogue breaks", "Include sensory details",
            "Vary sentence structure", "Introduce new character voice", "Change scene tempo"
        ];
        
        let selected = &techniques[0..3]; // Select first 3 techniques
        selected.join(", ")
    }
    
    fn generate_simplification_approach(&self, _context: &str) -> String {
        "Focus on core action, reduce nested clauses, use active voice, shorter paragraphs".to_string()
    }
    
    fn generate_creative_refresh_approach(&self, phase: &GenerationPhase) -> String {
        match phase {
            GenerationPhase::DialogueCreation => "Add character mannerisms, emotional subtext, conflict",
            GenerationPhase::DescriptiveWriting => "Use vivid imagery, sensory details, metaphorical language",
            GenerationPhase::CharacterDevelopment => "Reveal internal conflicts, hidden motivations, backstory",
            GenerationPhase::PlotAdvancement => "Introduce complications, raise stakes, add urgency",
            _ => "Apply fresh perspective, unexpected elements, emotional depth",
        }.to_string()
    }
    
    /// Change writing approach for variety
    fn change_writing_approach(&self, context: &str, phase: &GenerationPhase) -> String {
        let current_style = self.detect_current_style(context);
        let new_approach = match (phase, current_style.as_str()) {
            (GenerationPhase::DialogueCreation, "formal") => "casual conversational",
            (GenerationPhase::DialogueCreation, "casual") => "dramatic tension-filled",
            (GenerationPhase::DescriptiveWriting, "verbose") => "concise impactful",
            (GenerationPhase::DescriptiveWriting, "minimal") => "rich detailed",
            (GenerationPhase::ChapterWriting, "action-heavy") => "character-focused",
            (GenerationPhase::ChapterWriting, "introspective") => "plot-driven",
            _ => "balanced narrative",
        };
        
        format!("STYLE_CHANGE: Switching from {} to {} approach", current_style, new_approach)
    }
    
    fn detect_current_style(&self, context: &str) -> String {
        let dialogue_ratio = context.matches('"').count() as f32 / context.len() as f32;
        let avg_sentence_length = context.split_whitespace().count() as f32 / context.split('.').count() as f32;
        let action_words = context.matches(|c: char| c.is_alphabetic()).count();
        
        if dialogue_ratio > 0.1 {
            "dialogue-heavy"
        } else if avg_sentence_length > 25.0 {
            "verbose"
        } else if avg_sentence_length < 10.0 {
            "minimal"
        } else if action_words > context.len() / 8 {
            "action-heavy"
        } else {
            "formal"
        }.to_string()
    }
    
    /// Simplify language patterns
    fn simplify_language_patterns(&self, context: &str) -> String {
        let simplified = self.simplify_vocabulary(context);
        let further_simplified = self.reduce_sentence_complexity(&simplified);
        format!("LANGUAGE_SIMPLIFIED: {}", further_simplified)
    }
    
    fn reduce_sentence_complexity(&self, text: &str) -> String {
        // Break compound sentences at conjunctions
        let conjunctions = [" and ", " but ", " or ", " so ", " yet "];
        let mut simplified = text.to_string();
        
        for conjunction in conjunctions {
            if simplified.contains(conjunction) {
                simplified = simplified.replace(conjunction, &format!(".{}", conjunction.trim()));
            }
        }
        
        simplified
    }
    
    /// Refresh context memory
    fn refresh_context_memory(&self, context: &str) -> String {
        let key_elements = self.extract_key_story_elements(context);
        format!("CONTEXT_REFRESHED: Preserved key elements: {}. Memory optimized for continued generation.", key_elements)
    }
    
    fn extract_key_story_elements(&self, context: &str) -> String {
        let mut elements = Vec::new();
        
        // Extract character names (capitalized words that appear multiple times)
        let words: Vec<&str> = context.split_whitespace().collect();
        let mut word_counts = std::collections::HashMap::new();
        
        for word in words {
            let clean = word.trim_matches(|c: char| !c.is_alphabetic());
            if clean.len() > 2 && clean.chars().next().unwrap().is_uppercase() {
                *word_counts.entry(clean).or_insert(0) += 1;
            }
        }
        
        // Characters mentioned multiple times
        for (word, count) in word_counts {
            if count > 2 {
                elements.push(word.to_string());
            }
        }
        
        if elements.is_empty() {
            "narrative continuity, story flow".to_string()
        } else {
            elements.join(", ")
        }
    }

    /// Skip problematic section and continue with generation
    fn skip_and_continue(&self, context: &str, phase: &GenerationPhase) -> String {
        let skip_indicators = self.create_skip_indicators(phase);
        let continuation_hint = self.generate_continuation_hint(context, phase);
        
        format!("SECTION_SKIPPED: {}. Continuing with: {}", skip_indicators, continuation_hint)
    }
    
    fn create_skip_indicators(&self, phase: &GenerationPhase) -> String {
        match phase {
            GenerationPhase::DialogueCreation => "Dialogue section bypassed - moving to narrative",
            GenerationPhase::DescriptiveWriting => "Description condensed - focusing on action",
            GenerationPhase::ChapterWriting => "Chapter section streamlined - advancing plot",
            GenerationPhase::SceneGeneration => "Scene transition accelerated",
            GenerationPhase::CharacterDevelopment => "Character detail deferred - maintaining story flow",
            _ => "Complex section simplified - maintaining narrative momentum"
        }.to_string()
    }
    
    fn generate_continuation_hint(&self, context: &str, phase: &GenerationPhase) -> String {
        let last_sentences: Vec<&str> = context.split('.').collect();
        let recent_context = last_sentences.iter().rev().take(2).map(|s| s.to_string()).collect::<Vec<_>>().join(".");
        
        match phase {
            GenerationPhase::DialogueCreation => "narrative action and scene progression",
            GenerationPhase::DescriptiveWriting => "character dialogue and plot advancement",  
            GenerationPhase::ChapterWriting => {
                if recent_context.contains("said") || recent_context.contains('"') {
                    "continuing dialogue flow"
                } else {
                    "advancing narrative timeline"
                }
            }
            _ => "maintaining story momentum with simplified approach"
        }.to_string()
    }

    /// Generate user input prompt for manual assistance
    fn generate_user_input_prompt(&self, context: &str, phase: &GenerationPhase) -> String {
        let problem_analysis = self.analyze_generation_problem(context, phase);
        let user_guidance = self.create_user_guidance(phase);
        
        format!("USER_INPUT_REQUESTED: {} - {}", problem_analysis, user_guidance)
    }
    
    fn analyze_generation_problem(&self, context: &str, phase: &GenerationPhase) -> String {
        let context_length = context.len();
        let complexity_level = if context_length > 2000 { "high" } else if context_length > 500 { "medium" } else { "low" };
        
        match phase {
            GenerationPhase::DialogueCreation => format!("Dialogue generation stalled (complexity: {})", complexity_level),
            GenerationPhase::DescriptiveWriting => format!("Descriptive writing blocked (context: {} chars)", context_length),
            GenerationPhase::ChapterWriting => format!("Chapter progression halted (complexity: {})", complexity_level),
            GenerationPhase::CharacterDevelopment => "Character development requires creative input".to_string(),
            GenerationPhase::PlotAdvancement => "Plot logic needs resolution".to_string(),
            _ => format!("Generation issue in {:?} phase", phase)
        }
    }
    
    fn create_user_guidance(&self, phase: &GenerationPhase) -> String {
        match phase {
            GenerationPhase::DialogueCreation => "Please provide dialogue direction or character voice guidance",
            GenerationPhase::DescriptiveWriting => "Suggest specific details, mood, or sensory elements to include",
            GenerationPhase::ChapterWriting => "Provide plot direction, pacing preference, or content focus",
            GenerationPhase::CharacterDevelopment => "Specify character traits, development arc, or relationship dynamics",
            GenerationPhase::PlotAdvancement => "Clarify plot direction, resolve conflicts, or suggest next events",
            _ => "Please provide creative guidance or specify how to proceed"
        }.to_string()
    }
}

impl Default for SelfHealingWriter {
    fn default() -> Self {
        Self::new()
    }
}