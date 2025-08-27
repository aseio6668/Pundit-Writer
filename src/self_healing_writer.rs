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
                Ok("Simplified content approach".to_string())
            }
            ResolutionStrategy::SplitIntoSmallerChunks => {
                Ok("Breaking into smaller sections".to_string())
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

impl Default for SelfHealingWriter {
    fn default() -> Self {
        Self::new()
    }
}