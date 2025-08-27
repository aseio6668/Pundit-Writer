use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::cli_types::{Genre, WritingStyle, BookSize};
use crate::self_healing_writer::{GenerationPhase, ErrorCategory, ResolutionStrategy};
use crate::adaptive_learning_engine::{ContentIssue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSignature {
    pub signature_id: String,
    pub error_pattern: String,
    pub context_patterns: Vec<String>,
    pub typical_causes: Vec<String>,
    pub severity_level: SeverityLevel,
    pub frequency_score: f32,
    pub detection_confidence: f32,
    pub first_detected: DateTime<Utc>,
    pub last_occurrence: DateTime<Utc>,
    pub occurrence_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualErrorPattern {
    pub genre_specific: HashMap<Genre, Vec<ErrorSignature>>,
    pub style_specific: HashMap<WritingStyle, Vec<ErrorSignature>>,
    pub phase_specific: HashMap<GenerationPhase, Vec<ErrorSignature>>,
    pub size_specific: HashMap<BookSize, Vec<ErrorSignature>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPrediction {
    pub predicted_errors: Vec<PredictedError>,
    pub risk_factors: Vec<RiskFactor>,
    pub prevention_strategies: Vec<PreventionStrategy>,
    pub confidence_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedError {
    pub error_type: ErrorCategory,
    pub probability: f32,
    pub expected_phase: GenerationPhase,
    pub potential_triggers: Vec<String>,
    pub suggested_preventive_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: RiskType,
    pub description: String,
    pub impact_score: f32,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskType {
    ContextualComplexity,
    ModelLimitations,
    PromptAmbiguity,
    MemoryConstraints,
    CreativeExhaustion,
    UserExpectationMismatch,
    TechnicalConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventionStrategy {
    pub strategy_name: String,
    pub description: String,
    pub effectiveness_rating: f32,
    pub implementation_complexity: ComplexityLevel,
    pub prerequisites: Vec<String>,
    pub expected_impact: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct ErrorPatternRecognition {
    pub error_signatures: HashMap<String, ErrorSignature>,
    pub contextual_patterns: ContextualErrorPattern,
    pub pattern_learning_enabled: bool,
    pub detection_sensitivity: f32,
    pub minimum_pattern_threshold: u32,
}

impl ErrorPatternRecognition {
    pub fn new() -> Self {
        Self {
            error_signatures: HashMap::new(),
            contextual_patterns: ContextualErrorPattern {
                genre_specific: HashMap::new(),
                style_specific: HashMap::new(),
                phase_specific: HashMap::new(),
                size_specific: HashMap::new(),
            },
            pattern_learning_enabled: true,
            detection_sensitivity: 0.7,
            minimum_pattern_threshold: 3,
        }
    }

    pub fn analyze_error(
        &mut self,
        error_message: &str,
        context: &str,
        genre: &Genre,
        style: &WritingStyle,
        size: &BookSize,
        phase: &GenerationPhase,
        category: &ErrorCategory,
    ) -> ErrorAnalysisResult {
        // Extract error pattern from message
        let pattern = self.extract_error_pattern(error_message);
        let signature_id = self.generate_signature_id(&pattern, category);

        // Prepare data outside of borrow
        let context_pattern = self.extract_context_pattern(context);
        let probable_causes = self.identify_probable_causes(error_message, context, category);

        // Check if signature exists to determine if pattern is recognized
        let existing_count = self.error_signatures.get(&signature_id)
            .map(|s| s.occurrence_count)
            .unwrap_or(0);

        // Update or create error signature
        let signature = self.error_signatures
            .entry(signature_id.clone())
            .or_insert_with(|| ErrorSignature {
                signature_id: signature_id.clone(),
                error_pattern: pattern.clone(),
                context_patterns: Vec::new(),
                typical_causes: Vec::new(),
                severity_level: SeverityLevel::Medium,
                frequency_score: 0.0,
                detection_confidence: 0.5,
                first_detected: Utc::now(),
                last_occurrence: Utc::now(),
                occurrence_count: 0,
            });

        // Update signature values
        signature.occurrence_count += 1;
        signature.last_occurrence = Utc::now();
        
        // Store values needed for calculations
        let new_occurrence_count = signature.occurrence_count;
        let first_detected_time = signature.first_detected;
        
        // Add context pattern if not present
        if !signature.context_patterns.contains(&context_pattern) {
            signature.context_patterns.push(context_pattern);
        }

        // Update causes 
        signature.typical_causes = probable_causes;
        
        // Calculate all values outside of signature access
        let frequency_score = {
            let days_since_first = (Utc::now() - first_detected_time).num_days().max(1) as f32;
            let frequency = new_occurrence_count as f32 / days_since_first;
            (frequency / (frequency + 1.0)).min(1.0)
        };
        
        let severity_level = match category {
            ErrorCategory::MemoryOverflow | ErrorCategory::SystemTimeout => SeverityLevel::High,
            ErrorCategory::CreativeBlock | ErrorCategory::TemporalInconsistency => SeverityLevel::Medium,
            ErrorCategory::FormatError | ErrorCategory::ParsingError => SeverityLevel::Low,
            ErrorCategory::Unknown => SeverityLevel::Medium,
            _ => SeverityLevel::Medium,
        };
        
        let escalated_severity = match (&severity_level, new_occurrence_count) {
            (SeverityLevel::Low, count) if count > 10 => SeverityLevel::Medium,
            (SeverityLevel::Medium, count) if count > 5 => SeverityLevel::High,
            (SeverityLevel::High, count) if count > 3 => SeverityLevel::Critical,
            _ => severity_level,
        };

        let detection_confidence = {
            let context_diversity = signature.context_patterns.len() as f32;
            let occurrence_weight = (new_occurrence_count as f32).log10().max(0.0);
            let pattern_clarity = if signature.error_pattern.len() > 10 { 0.8 } else { 0.4 };
            ((context_diversity * 0.3 + occurrence_weight * 0.4 + pattern_clarity * 0.3) / 2.0).min(1.0)
        };

        let suggested_actions = match category {
            ErrorCategory::MemoryOverflow => vec![
                "Clear memory buffers".to_string(),
                "Reduce context size".to_string(),
                "Split generation into smaller chunks".to_string(),
            ],
            ErrorCategory::CreativeBlock => vec![
                "Change writing style or persona".to_string(),
                "Introduce creative prompts".to_string(),
                "Take a brief pause for reflection".to_string(),
            ],
            ErrorCategory::SystemTimeout => vec![
                "Retry with shorter prompt".to_string(),
                "Implement exponential backoff".to_string(),
                "Switch to alternative model if available".to_string(),
            ],
            _ => vec![
                "Apply standard retry logic".to_string(),
                "Log error for further analysis".to_string(),
            ],
        };

        let prevention_recommendations = match escalated_severity {
            SeverityLevel::Critical => vec![
                "Implement pre-generation checks".to_string(),
                "Add early warning system".to_string(),
                "Create dedicated handling for this pattern".to_string(),
            ],
            SeverityLevel::High => vec![
                "Monitor conditions that trigger this error".to_string(),
                "Implement preventive measures".to_string(),
            ],
            _ => vec![
                "Continue monitoring".to_string(),
                "Consider optimization if frequency increases".to_string(),
            ],
        };

        // Update signature with computed values
        signature.frequency_score = frequency_score;
        signature.severity_level = escalated_severity.clone();
        signature.detection_confidence = detection_confidence;

        // Create the result
        let result = ErrorAnalysisResult {
            signature_id: signature_id.clone(),
            pattern_recognized: new_occurrence_count > 1,
            severity: escalated_severity,
            confidence: detection_confidence,
            suggested_actions,
            prevention_recommendations,
        };

        // Update contextual patterns (clone signature to avoid borrow issues)
        let signature_clone = signature.clone();
        self.update_contextual_patterns(genre, style, size, phase, signature_clone);

        result
    }

    pub fn predict_potential_errors(
        &self,
        genre: &Genre,
        style: &WritingStyle,
        size: &BookSize,
        phase: &GenerationPhase,
        prompt_content: &str,
    ) -> ErrorPrediction {
        let mut predicted_errors = Vec::new();
        let mut risk_factors = Vec::new();
        let mut prevention_strategies = Vec::new();

        // Analyze prompt for risk factors
        risk_factors.extend(self.analyze_prompt_risks(prompt_content));
        
        // Check genre-specific patterns
        if let Some(genre_signatures) = self.contextual_patterns.genre_specific.get(genre) {
            for signature in genre_signatures {
                if signature.frequency_score > 0.3 {
                    predicted_errors.push(PredictedError {
                        error_type: self.signature_to_error_category(signature),
                        probability: signature.frequency_score * signature.detection_confidence,
                        expected_phase: phase.clone(),
                        potential_triggers: signature.typical_causes.clone(),
                        suggested_preventive_actions: self.generate_preventive_actions(signature),
                    });
                }
            }
        }

        // Check style-specific patterns
        if let Some(style_signatures) = self.contextual_patterns.style_specific.get(style) {
            for signature in style_signatures {
                if signature.frequency_score > 0.3 {
                    // Avoid duplicates
                    let error_type = self.signature_to_error_category(signature);
                    if !predicted_errors.iter().any(|pe| pe.error_type == error_type) {
                        predicted_errors.push(PredictedError {
                            error_type,
                            probability: signature.frequency_score * signature.detection_confidence * 0.8, // Slightly lower weight for style
                            expected_phase: phase.clone(),
                            potential_triggers: signature.typical_causes.clone(),
                            suggested_preventive_actions: self.generate_preventive_actions(signature),
                        });
                    }
                }
            }
        }

        // Check phase-specific patterns
        if let Some(phase_signatures) = self.contextual_patterns.phase_specific.get(phase) {
            for signature in phase_signatures {
                if signature.frequency_score > 0.4 {
                    let error_type = self.signature_to_error_category(signature);
                    if !predicted_errors.iter().any(|pe| pe.error_type == error_type) {
                        predicted_errors.push(PredictedError {
                            error_type,
                            probability: signature.frequency_score * signature.detection_confidence,
                            expected_phase: phase.clone(),
                            potential_triggers: signature.typical_causes.clone(),
                            suggested_preventive_actions: self.generate_preventive_actions(signature),
                        });
                    }
                }
            }
        }

        // Generate prevention strategies
        prevention_strategies.extend(self.create_prevention_strategies(&predicted_errors, &risk_factors));

        let confidence_level = if predicted_errors.is_empty() { 
            0.3 
        } else { 
            predicted_errors.iter().map(|pe| pe.probability).sum::<f32>() / predicted_errors.len() as f32 
        };

        ErrorPrediction {
            predicted_errors,
            risk_factors,
            prevention_strategies,
            confidence_level,
        }
    }

    pub fn get_error_intelligence_report(&self) -> Vec<String> {
        let mut report = Vec::new();

        report.push("🧠 Error Intelligence Report".to_string());
        report.push("═══════════════════════════".to_string());

        // Most frequent error patterns
        let mut frequent_errors: Vec<_> = self.error_signatures.values().collect();
        frequent_errors.sort_by(|a, b| b.occurrence_count.cmp(&a.occurrence_count));

        report.push("\n📊 Most Frequent Error Patterns:".to_string());
        for (i, signature) in frequent_errors.iter().take(5).enumerate() {
            report.push(format!(
                "{}. {} (occurred {} times, severity: {:?})",
                i + 1,
                signature.error_pattern.chars().take(60).collect::<String>(),
                signature.occurrence_count,
                signature.severity_level
            ));
        }

        // High-risk patterns
        let high_risk: Vec<_> = self.error_signatures.values()
            .filter(|s| s.severity_level == SeverityLevel::High || s.severity_level == SeverityLevel::Critical)
            .collect();

        if !high_risk.is_empty() {
            report.push("\n⚠️ High-Risk Error Patterns:".to_string());
            for signature in high_risk.iter().take(3) {
                report.push(format!(
                    "• {} (confidence: {:.0}%)",
                    signature.error_pattern.chars().take(80).collect::<String>(),
                    signature.detection_confidence * 100.0
                ));
            }
        }

        // Genre-specific insights
        report.push("\n🎭 Genre-Specific Error Patterns:".to_string());
        for (genre, signatures) in &self.contextual_patterns.genre_specific {
            if !signatures.is_empty() {
                let avg_severity = signatures.iter()
                    .map(|s| match s.severity_level {
                        SeverityLevel::Low => 1,
                        SeverityLevel::Medium => 2,
                        SeverityLevel::High => 3,
                        SeverityLevel::Critical => 4,
                    })
                    .sum::<u32>() as f32 / signatures.len() as f32;
                
                report.push(format!(
                    "• {}: {} patterns (avg severity: {:.1}/4)",
                    format!("{:?}", genre),
                    signatures.len(),
                    avg_severity
                ));
            }
        }

        report
    }

    fn extract_error_pattern(&self, error_message: &str) -> String {
        // Extract meaningful pattern from error message
        let clean_message = error_message
            .to_lowercase()
            .replace(&[':', ';', '"', '\''][..], "")
            .trim()
            .to_string();

        // Look for key error indicators
        let patterns = [
            ("timeout", "generation_timeout"),
            ("memory", "memory_issue"),
            ("too short", "content_length_insufficient"),
            ("parsing", "parsing_failure"),
            ("network", "network_error"),
            ("creative", "creative_block"),
            ("character", "character_consistency"),
            ("plot", "plot_logic_error"),
            ("format", "formatting_issue"),
            ("boundary", "text_boundary_error"),
        ];

        for (keyword, pattern) in patterns {
            if clean_message.contains(keyword) {
                return pattern.to_string();
            }
        }

        // Generic pattern based on first few words
        clean_message
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join("_")
    }

    fn extract_context_pattern(&self, context: &str) -> String {
        // Extract meaningful context patterns
        let words: Vec<&str> = context.split_whitespace().collect();
        let word_count = words.len();
        
        let context_indicators = [
            ("chapter", "chapter_context"),
            ("scene", "scene_context"),
            ("dialogue", "dialogue_context"),
            ("description", "descriptive_context"),
            ("character", "character_context"),
        ];

        for (keyword, pattern) in context_indicators {
            if context.to_lowercase().contains(keyword) {
                return format!("{}_{}words", pattern, word_count / 100 * 100); // Round to nearest 100
            }
        }

        format!("generic_context_{}words", word_count / 100 * 100)
    }

    fn identify_probable_causes(&self, _error_message: &str, context: &str, category: &ErrorCategory) -> Vec<String> {
        let mut causes = Vec::new();

        match category {
            ErrorCategory::MemoryOverflow => {
                causes.push("Context too large".to_string());
                causes.push("Accumulated memory usage".to_string());
                if context.len() > 5000 {
                    causes.push("Context exceeds recommended size".to_string());
                }
            }
            ErrorCategory::CreativeBlock => {
                causes.push("Repetitive prompting".to_string());
                causes.push("Insufficient creative context".to_string());
                causes.push("Over-constrained parameters".to_string());
            }
            ErrorCategory::SystemTimeout => {
                causes.push("Complex generation request".to_string());
                causes.push("Server overload".to_string());
                causes.push("Network latency".to_string());
            }
            ErrorCategory::ParsingError => {
                causes.push("Malformed prompt structure".to_string());
                causes.push("Special character issues".to_string());
                causes.push("Encoding problems".to_string());
            }
            _ => {
                causes.push("Unknown cause".to_string());
            }
        }

        causes
    }

    fn assess_severity(&self, _error_message: &str, category: &ErrorCategory, occurrence_count: u32) -> SeverityLevel {
        let base_severity = match category {
            ErrorCategory::MemoryOverflow | ErrorCategory::SystemTimeout => SeverityLevel::High,
            ErrorCategory::CreativeBlock | ErrorCategory::TemporalInconsistency => SeverityLevel::Medium,
            ErrorCategory::FormatError | ErrorCategory::ParsingError => SeverityLevel::Low,
            ErrorCategory::Unknown => SeverityLevel::Medium,
            _ => SeverityLevel::Medium,
        };

        // Escalate severity based on frequency
        match (&base_severity, occurrence_count) {
            (SeverityLevel::Low, count) if count > 10 => SeverityLevel::Medium,
            (SeverityLevel::Medium, count) if count > 5 => SeverityLevel::High,
            (SeverityLevel::High, count) if count > 3 => SeverityLevel::Critical,
            _ => base_severity,
        }
    }

    fn calculate_frequency_score(&self, occurrence_count: u32, first_detected: &DateTime<Utc>) -> f32 {
        let days_since_first = (Utc::now() - *first_detected).num_days().max(1) as f32;
        let frequency = occurrence_count as f32 / days_since_first;
        
        // Normalize to 0-1 scale
        (frequency / (frequency + 1.0)).min(1.0)
    }

    fn calculate_detection_confidence(&self, signature: &ErrorSignature) -> f32 {
        let context_diversity = signature.context_patterns.len() as f32;
        let occurrence_weight = (signature.occurrence_count as f32).log10().max(0.0);
        let pattern_clarity = if signature.error_pattern.len() > 10 { 0.8 } else { 0.4 };
        
        ((context_diversity * 0.3 + occurrence_weight * 0.4 + pattern_clarity * 0.3) / 2.0).min(1.0)
    }

    fn update_contextual_patterns(
        &mut self,
        genre: &Genre,
        style: &WritingStyle,
        size: &BookSize,
        phase: &GenerationPhase,
        signature: ErrorSignature,
    ) {
        // Update genre-specific patterns
        self.contextual_patterns
            .genre_specific
            .entry(genre.clone())
            .or_insert_with(Vec::new)
            .push(signature.clone());

        // Update style-specific patterns
        self.contextual_patterns
            .style_specific
            .entry(style.clone())
            .or_insert_with(Vec::new)
            .push(signature.clone());

        // Update phase-specific patterns
        self.contextual_patterns
            .phase_specific
            .entry(phase.clone())
            .or_insert_with(Vec::new)
            .push(signature.clone());

        // Update size-specific patterns
        self.contextual_patterns
            .size_specific
            .entry(size.clone())
            .or_insert_with(Vec::new)
            .push(signature);
    }

    fn suggest_immediate_actions(&self, _signature: &ErrorSignature, category: &ErrorCategory) -> Vec<String> {
        let mut actions = Vec::new();

        match category {
            ErrorCategory::MemoryOverflow => {
                actions.push("Clear memory buffers".to_string());
                actions.push("Reduce context size".to_string());
                actions.push("Split generation into smaller chunks".to_string());
            }
            ErrorCategory::CreativeBlock => {
                actions.push("Change writing style or persona".to_string());
                actions.push("Introduce creative prompts".to_string());
                actions.push("Take a brief pause for reflection".to_string());
            }
            ErrorCategory::SystemTimeout => {
                actions.push("Retry with shorter prompt".to_string());
                actions.push("Implement exponential backoff".to_string());
                actions.push("Switch to alternative model if available".to_string());
            }
            _ => {
                actions.push("Apply standard retry logic".to_string());
                actions.push("Log error for further analysis".to_string());
            }
        }

        actions
    }

    fn generate_prevention_recommendations(&self, signature: &ErrorSignature) -> Vec<String> {
        let mut recommendations = Vec::new();

        match signature.severity_level {
            SeverityLevel::Critical => {
                recommendations.push("Implement pre-generation checks".to_string());
                recommendations.push("Add early warning system".to_string());
                recommendations.push("Create dedicated handling for this pattern".to_string());
            }
            SeverityLevel::High => {
                recommendations.push("Monitor conditions that trigger this error".to_string());
                recommendations.push("Implement preventive measures".to_string());
            }
            _ => {
                recommendations.push("Continue monitoring".to_string());
                recommendations.push("Consider optimization if frequency increases".to_string());
            }
        }

        recommendations
    }

    fn analyze_prompt_risks(&self, prompt: &str) -> Vec<RiskFactor> {
        let mut risks = Vec::new();
        
        if prompt.len() > 2000 {
            risks.push(RiskFactor {
                factor_type: RiskType::ContextualComplexity,
                description: "Prompt is very long and complex".to_string(),
                impact_score: 0.7,
                mitigation_strategies: vec![
                    "Break into smaller segments".to_string(),
                    "Simplify language and structure".to_string(),
                ],
            });
        }

        if prompt.split_whitespace().count() < 10 {
            risks.push(RiskFactor {
                factor_type: RiskType::PromptAmbiguity,
                description: "Prompt may be too brief or ambiguous".to_string(),
                impact_score: 0.5,
                mitigation_strategies: vec![
                    "Add more context and detail".to_string(),
                    "Provide clearer instructions".to_string(),
                ],
            });
        }

        risks
    }

    fn signature_to_error_category(&self, signature: &ErrorSignature) -> ErrorCategory {
        // Map error patterns back to categories
        if signature.error_pattern.contains("memory") {
            ErrorCategory::MemoryOverflow
        } else if signature.error_pattern.contains("creative") {
            ErrorCategory::CreativeBlock
        } else if signature.error_pattern.contains("timeout") {
            ErrorCategory::SystemTimeout
        } else if signature.error_pattern.contains("parsing") {
            ErrorCategory::ParsingError
        } else {
            ErrorCategory::Unknown
        }
    }

    fn generate_preventive_actions(&self, signature: &ErrorSignature) -> Vec<String> {
        signature.typical_causes
            .iter()
            .map(|cause| match cause.as_str() {
                "Context too large" => "Optimize context size before generation",
                "Repetitive prompting" => "Vary prompt structure and approach",
                "Server overload" => "Implement retry with backoff",
                _ => "Monitor conditions and apply standard prevention"
            })
            .map(String::from)
            .collect()
    }

    fn create_prevention_strategies(&self, predicted_errors: &[PredictedError], _risk_factors: &[RiskFactor]) -> Vec<PreventionStrategy> {
        let mut strategies = Vec::new();

        if predicted_errors.iter().any(|pe| matches!(pe.error_type, ErrorCategory::MemoryOverflow)) {
            strategies.push(PreventionStrategy {
                strategy_name: "Memory Optimization".to_string(),
                description: "Proactively manage memory usage during generation".to_string(),
                effectiveness_rating: 0.8,
                implementation_complexity: ComplexityLevel::Medium,
                prerequisites: vec!["Memory monitoring system".to_string()],
                expected_impact: vec!["Reduced memory-related failures".to_string()],
            });
        }

        if predicted_errors.iter().any(|pe| matches!(pe.error_type, ErrorCategory::CreativeBlock)) {
            strategies.push(PreventionStrategy {
                strategy_name: "Creative Diversity Enhancement".to_string(),
                description: "Maintain creative variety to prevent blocks".to_string(),
                effectiveness_rating: 0.7,
                implementation_complexity: ComplexityLevel::Low,
                prerequisites: vec!["Alternative persona system".to_string()],
                expected_impact: vec!["Improved creative flow".to_string()],
            });
        }

        strategies
    }

    fn generate_signature_id(&self, pattern: &str, category: &ErrorCategory) -> String {
        format!("{:?}_{}", category, pattern.chars().take(20).collect::<String>())
    }
}

#[derive(Debug, Clone)]
pub struct ErrorAnalysisResult {
    pub signature_id: String,
    pub pattern_recognized: bool,
    pub severity: SeverityLevel,
    pub confidence: f32,
    pub suggested_actions: Vec<String>,
    pub prevention_recommendations: Vec<String>,
}

impl Default for ErrorPatternRecognition {
    fn default() -> Self {
        Self::new()
    }
}