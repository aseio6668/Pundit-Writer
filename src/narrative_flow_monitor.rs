use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use crate::cli_types::{Genre, WritingStyle};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeFlowMonitor {
    confusion_threshold: f32,
    repetition_threshold: f32,
    readability_threshold: f32,
    complexity_window: usize,
    recent_content_buffer: VecDeque<ContentAnalysis>,
    stuck_patterns: Vec<StuckPattern>,
    pivot_history: Vec<NarrativePivot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    timestamp: DateTime<Utc>,
    content: String,
    word_count: usize,
    sentence_complexity: f32,
    repetition_score: f32,
    readability_score: f32,
    confusion_indicators: Vec<ConfusionIndicator>,
    logical_coherence: f32,
    narrative_momentum: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfusionIndicator {
    pub indicator_type: ConfusionType,
    pub severity: f32,
    pub description: String,
    pub text_sample: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfusionType {
    LogicalInconsistency,
    CircularReasoning,
    OverComplexification,
    RepetitiveStructure,
    ConvolutedSentences,
    UnresolvedPlotThreads,
    CharacterInconsistency,
    TemporalConfusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StuckPattern {
    pattern_id: String,
    pattern_type: StuckType,
    trigger_phrases: Vec<String>,
    complexity_markers: Vec<String>,
    detection_count: u32,
    last_detected: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StuckType {
    LogicalLoop,
    PlotKnot,
    CharacterMotivationTangle,
    WorldBuildingOverload,
    ConceptualDeadEnd,
    TemporalParadox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativePivot {
    pivot_id: String,
    timestamp: DateTime<Utc>,
    trigger_reason: String,
    abandoned_content: String,
    extracted_elements: Vec<ExtractedElement>,
    pivot_strategy: PivotStrategy,
    new_direction: String,
    success_rating: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedElement {
    pub element_type: ElementType,
    pub content: String,
    pub importance_score: f32,
    pub reuse_potential: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    CharacterName,
    LocationName,
    ConceptName,
    ImportantEvent,
    Relationship,
    Atmosphere,
    Theme,
    SymbolicElement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PivotStrategy {
    CompleteShift,          // Completely new direction
    ElementReuse,           // Use extracted elements in new context
    TemporalJump,          // Jump forward/backward in time
    PerspectiveShift,      // Change POV character
    GenreBlend,            // Introduce different genre elements
    MetaResolution,        // Address the complexity meta-textually
    SummaryIntegration,    // Compress the complex part into summary
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowDecision {
    pub should_pivot: bool,
    pub confidence: f32,
    pub detected_issues: Vec<ConfusionIndicator>,
    pub recommended_strategy: Option<PivotStrategy>,
    pub extracted_elements: Vec<ExtractedElement>,
    pub pivot_prompt: Option<String>,
    pub explanation: String,
}

impl Default for NarrativeFlowMonitor {
    fn default() -> Self {
        Self {
            confusion_threshold: 0.7,
            repetition_threshold: 0.6,
            readability_threshold: 0.4,
            complexity_window: 5,
            recent_content_buffer: VecDeque::with_capacity(10),
            stuck_patterns: Vec::new(),
            pivot_history: Vec::new(),
        }
    }
}

impl NarrativeFlowMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze_content(&mut self, content: &str, chapter_context: &str) -> Result<FlowDecision> {
        let analysis = self.perform_content_analysis(content)?;
        
        // Add to buffer
        self.recent_content_buffer.push_back(analysis.clone());
        if self.recent_content_buffer.len() > 10 {
            self.recent_content_buffer.pop_front();
        }

        // Check for stuck patterns
        let stuck_detected = self.detect_stuck_patterns(&analysis);
        
        // Evaluate if we should pivot
        let should_pivot = self.should_trigger_pivot(&analysis, stuck_detected);
        
        if should_pivot {
            let pivot_strategy = self.determine_pivot_strategy(&analysis);
            let extracted_elements = self.extract_reusable_elements(content);
            let pivot_prompt = self.generate_pivot_prompt(&pivot_strategy, &extracted_elements, chapter_context);
            
            Ok(FlowDecision {
                should_pivot: true,
                confidence: self.calculate_pivot_confidence(&analysis),
                detected_issues: analysis.confusion_indicators,
                recommended_strategy: Some(pivot_strategy.clone()),
                extracted_elements,
                pivot_prompt,
                explanation: format!("Flow intervention needed: {:?}", pivot_strategy),
            })
        } else {
            Ok(FlowDecision {
                should_pivot: false,
                confidence: 1.0 - analysis.confusion_indicators.len() as f32 * 0.2,
                detected_issues: analysis.confusion_indicators,
                recommended_strategy: None,
                extracted_elements: Vec::new(),
                pivot_prompt: None,
                explanation: "Flow is acceptable, no intervention needed".to_string(),
            })
        }
    }

    fn perform_content_analysis(&self, content: &str) -> Result<ContentAnalysis> {
        let word_count = content.split_whitespace().count();
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        
        let sentence_complexity = self.calculate_sentence_complexity(&sentences);
        let repetition_score = self.calculate_repetition_score(content);
        let readability_score = self.calculate_readability_score(content);
        let confusion_indicators = self.detect_confusion_indicators(content);
        let logical_coherence = self.assess_logical_coherence(content);
        let narrative_momentum = self.assess_narrative_momentum(content);

        Ok(ContentAnalysis {
            timestamp: Utc::now(),
            content: content.to_string(),
            word_count,
            sentence_complexity,
            repetition_score,
            readability_score,
            confusion_indicators,
            logical_coherence,
            narrative_momentum,
        })
    }

    fn calculate_sentence_complexity(&self, sentences: &[&str]) -> f32 {
        if sentences.is_empty() {
            return 0.0;
        }

        let mut total_complexity = 0.0;
        for sentence in sentences {
            let words = sentence.split_whitespace().count();
            let clauses = sentence.matches(',').count() + 1;
            let nested_structures = sentence.matches('(').count() + sentence.matches('[').count();
            
            // Complex sentence indicators
            let complexity_words = ["however", "therefore", "nevertheless", "consequently", "furthermore", "moreover"];
            let complex_word_count = complexity_words.iter()
                .map(|&word| sentence.matches(word).count())
                .sum::<usize>();
            
            let sentence_complexity = (words as f32 * 0.1) + 
                                    (clauses as f32 * 0.3) + 
                                    (nested_structures as f32 * 0.5) +
                                    (complex_word_count as f32 * 0.4);
            
            total_complexity += sentence_complexity;
        }
        
        total_complexity / sentences.len() as f32
    }

    fn calculate_repetition_score(&self, content: &str) -> f32 {
        let words: Vec<&str> = content.split_whitespace().collect();
        if words.len() < 10 {
            return 0.0;
        }

        // Check for word repetition patterns
        let mut word_counts = HashMap::new();
        for word in &words {
            let clean_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
            if clean_word.len() > 3 { // Ignore very short words
                *word_counts.entry(clean_word).or_insert(0) += 1;
            }
        }

        // Calculate repetition patterns
        let mut repetition_score = 0.0;
        for (_, count) in word_counts {
            if count > 2 {
                repetition_score += (count - 2) as f32 * 0.1;
            }
        }

        // Check for phrase repetition
        let phrases: Vec<String> = words.windows(3)
            .map(|window| window.join(" ").to_lowercase())
            .collect();
        
        let mut phrase_counts = HashMap::new();
        for phrase in phrases {
            *phrase_counts.entry(phrase).or_insert(0) += 1;
        }

        for (_, count) in phrase_counts {
            if count > 1 {
                repetition_score += count as f32 * 0.3;
            }
        }

        (repetition_score / words.len() as f32).min(1.0)
    }

    fn calculate_readability_score(&self, content: &str) -> f32 {
        let words: Vec<&str> = content.split_whitespace().collect();
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        
        if words.is_empty() || sentences.is_empty() {
            return 1.0;
        }

        let avg_words_per_sentence = words.len() as f32 / sentences.len() as f32;
        let complex_words = words.iter()
            .filter(|word| word.len() > 6)
            .count() as f32;
        let complex_word_ratio = complex_words / words.len() as f32;

        // Simplified readability score (inverse of complexity)
        let readability = 1.0 - (avg_words_per_sentence / 30.0 + complex_word_ratio).min(1.0);
        readability.max(0.0)
    }

    fn detect_confusion_indicators(&self, content: &str) -> Vec<ConfusionIndicator> {
        let mut indicators = Vec::new();

        // Detect logical inconsistencies
        let contradiction_patterns = [
            ("but", "however"), ("although", "despite"), ("while", "whereas")
        ];
        
        for (word1, word2) in &contradiction_patterns {
            if content.contains(word1) && content.contains(word2) {
                let proximity = self.check_word_proximity(content, word1, word2);
                if proximity < 50 { // Words are close together
                    indicators.push(ConfusionIndicator {
                        indicator_type: ConfusionType::LogicalInconsistency,
                        severity: 0.7,
                        description: format!("Contradictory logic words '{}' and '{}' in close proximity", word1, word2),
                        text_sample: self.extract_sample_around_words(content, word1, word2),
                    });
                }
            }
        }

        // Detect circular reasoning
        let circular_phrases = [
            "because of this", "due to the fact", "the reason being", "as a result of which"
        ];
        
        for phrase in &circular_phrases {
            let count = content.matches(phrase).count();
            if count > 2 {
                indicators.push(ConfusionIndicator {
                    indicator_type: ConfusionType::CircularReasoning,
                    severity: 0.6,
                    description: format!("Circular reasoning pattern detected with '{}'", phrase),
                    text_sample: self.extract_sample_around_phrase(content, phrase),
                });
            }
        }

        // Detect overcomplication
        let complexity_markers = [
            "on the other hand", "furthermore", "nevertheless", "consequently", "notwithstanding"
        ];
        
        let complexity_count = complexity_markers.iter()
            .map(|marker| content.matches(marker).count())
            .sum::<usize>();
        
        if complexity_count > content.split_whitespace().count() / 20 {
            indicators.push(ConfusionIndicator {
                indicator_type: ConfusionType::OverComplexification,
                severity: 0.8,
                description: "Excessive use of complex transitional phrases".to_string(),
                text_sample: self.extract_complexity_sample(content, &complexity_markers),
            });
        }

        // Detect repetitive structures
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        let mut structure_patterns = HashMap::new();
        
        for sentence in &sentences {
            let words: Vec<&str> = sentence.split_whitespace().collect();
            if words.len() > 3 {
                let structure = format!("{} ... {}", words[0], words[words.len()-1]);
                *structure_patterns.entry(structure).or_insert(0) += 1;
            }
        }

        for (pattern, count) in structure_patterns {
            if count > 2 {
                indicators.push(ConfusionIndicator {
                    indicator_type: ConfusionType::RepetitiveStructure,
                    severity: 0.5,
                    description: format!("Repetitive sentence structure: {}", pattern),
                    text_sample: pattern,
                });
            }
        }

        indicators
    }

    fn assess_logical_coherence(&self, content: &str) -> f32 {
        let sentences: Vec<&str> = content.split(&['.', '!', '?'][..]).collect();
        if sentences.len() < 2 {
            return 1.0;
        }

        let mut coherence_score = 1.0;
        
        // Check for temporal consistency
        let time_indicators = ["then", "next", "after", "before", "meanwhile", "suddenly"];
        let time_conflicts = self.detect_temporal_conflicts(content, &time_indicators);
        coherence_score -= time_conflicts * 0.2;

        // Check for pronoun consistency
        let pronoun_issues = self.detect_pronoun_inconsistencies(content);
        coherence_score -= pronoun_issues * 0.15;

        // Check for logical flow
        let logic_breaks = self.detect_logic_breaks(content);
        coherence_score -= logic_breaks * 0.25;

        coherence_score.max(0.0)
    }

    fn assess_narrative_momentum(&self, content: &str) -> f32 {
        let action_words = ["suddenly", "quickly", "immediately", "rushed", "ran", "jumped", "shouted"];
        let stagnation_words = ["pondered", "considered", "thought", "wondered", "reflected", "deliberated"];
        
        let action_count = action_words.iter()
            .map(|word| content.matches(word).count())
            .sum::<usize>();
            
        let stagnation_count = stagnation_words.iter()
            .map(|word| content.matches(word).count())
            .sum::<usize>();

        let total_words = content.split_whitespace().count();
        
        if total_words == 0 {
            return 0.5;
        }

        let action_ratio = action_count as f32 / total_words as f32;
        let stagnation_ratio = stagnation_count as f32 / total_words as f32;
        
        // Higher momentum when more action, less stagnation
        (action_ratio - stagnation_ratio + 0.5).max(0.0).min(1.0)
    }

    fn should_trigger_pivot(&self, analysis: &ContentAnalysis, stuck_detected: bool) -> bool {
        if stuck_detected {
            return true;
        }

        // Check recent trend
        if self.recent_content_buffer.len() >= self.complexity_window {
            let recent_complexity: f32 = self.recent_content_buffer
                .iter()
                .rev()
                .take(self.complexity_window)
                .map(|a| a.sentence_complexity)
                .sum::<f32>() / self.complexity_window as f32;

            let recent_repetition: f32 = self.recent_content_buffer
                .iter()
                .rev()
                .take(self.complexity_window)
                .map(|a| a.repetition_score)
                .sum::<f32>() / self.complexity_window as f32;

            if recent_complexity > self.confusion_threshold || 
               recent_repetition > self.repetition_threshold ||
               analysis.readability_score < self.readability_threshold {
                return true;
            }
        }

        // Check for multiple confusion indicators
        analysis.confusion_indicators.len() > 2 ||
        analysis.confusion_indicators.iter().any(|ci| ci.severity > 0.8)
    }

    fn determine_pivot_strategy(&self, analysis: &ContentAnalysis) -> PivotStrategy {
        let dominant_issue = analysis.confusion_indicators
            .iter()
            .max_by(|a, b| a.severity.partial_cmp(&b.severity).unwrap());

        match dominant_issue.map(|ci| &ci.indicator_type) {
            Some(ConfusionType::LogicalInconsistency) => PivotStrategy::MetaResolution,
            Some(ConfusionType::CircularReasoning) => PivotStrategy::TemporalJump,
            Some(ConfusionType::OverComplexification) => PivotStrategy::SummaryIntegration,
            Some(ConfusionType::RepetitiveStructure) => PivotStrategy::PerspectiveShift,
            Some(ConfusionType::ConvolutedSentences) => PivotStrategy::CompleteShift,
            Some(ConfusionType::UnresolvedPlotThreads) => PivotStrategy::ElementReuse,
            Some(ConfusionType::CharacterInconsistency) => PivotStrategy::PerspectiveShift,
            Some(ConfusionType::TemporalConfusion) => PivotStrategy::TemporalJump,
            None => {
                // Choose based on narrative momentum
                if analysis.narrative_momentum < 0.3 {
                    PivotStrategy::GenreBlend
                } else {
                    PivotStrategy::ElementReuse
                }
            }
        }
    }

    fn extract_reusable_elements(&self, content: &str) -> Vec<ExtractedElement> {
        let mut elements = Vec::new();

        // Extract character names (capitalized words that appear multiple times)
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut name_counts = HashMap::new();
        
        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic());
            if clean_word.len() > 2 && clean_word.chars().next().unwrap().is_uppercase() {
                *name_counts.entry(clean_word).or_insert(0) += 1;
            }
        }

        for (name, count) in name_counts {
            if count > 1 {
                elements.push(ExtractedElement {
                    element_type: ElementType::CharacterName,
                    content: name.to_string(),
                    importance_score: (count as f32 * 0.1).min(1.0),
                    reuse_potential: 0.8,
                });
            }
        }

        // Extract location indicators
        let location_markers = ["in the", "at the", "near the", "inside", "outside", "within"];
        for marker in &location_markers {
            if let Some(pos) = content.find(marker) {
                let after_marker = &content[pos + marker.len()..];
                if let Some(location_end) = after_marker.find(&['.', ',', ';'][..]) {
                    let location = after_marker[..location_end].trim();
                    if location.len() > 3 && location.len() < 50 {
                        elements.push(ExtractedElement {
                            element_type: ElementType::LocationName,
                            content: location.to_string(),
                            importance_score: 0.6,
                            reuse_potential: 0.9,
                        });
                    }
                }
            }
        }

        // Extract atmospheric elements
        let mood_words = ["dark", "bright", "cold", "warm", "mysterious", "cheerful", "gloomy", "peaceful"];
        for word in &mood_words {
            if content.contains(word) {
                elements.push(ExtractedElement {
                    element_type: ElementType::Atmosphere,
                    content: word.to_string(),
                    importance_score: 0.4,
                    reuse_potential: 0.7,
                });
            }
        }

        elements
    }

    fn generate_pivot_prompt(&self, strategy: &PivotStrategy, elements: &[ExtractedElement], chapter_context: &str) -> Option<String> {
        match strategy {
            PivotStrategy::CompleteShift => {
                Some(format!(
                    "The narrative has become too complex. Let's completely shift direction. Take a creative leap and write about something entirely different, but you may reference these elements if they feel natural: {}. New chapter focus: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
            
            PivotStrategy::ElementReuse => {
                Some(format!(
                    "The story is getting tangled in complexity. Let's simplify and focus on these key elements: {}. Create a new scene or chapter that uses these elements in a fresh, straightforward way. Context: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
            
            PivotStrategy::TemporalJump => {
                Some(format!(
                    "The current timeline is becoming confusing. Let's jump to a different time - either significantly earlier or later. You can reference these elements from the previous timeframe: {}. New temporal setting for: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
            
            PivotStrategy::PerspectiveShift => {
                Some(format!(
                    "The current perspective is getting muddled. Switch to a completely different character's point of view. They might know about these elements from the previous perspective: {}. New viewpoint for: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
            
            PivotStrategy::SummaryIntegration => {
                Some(format!(
                    "The plot has become too convoluted to continue directly. Summarize the complex situation briefly, then move forward with a cleaner narrative. Key elements to preserve: {}. Continue with: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
            
            PivotStrategy::GenreBlend => {
                Some(format!(
                    "The story needs fresh energy. Introduce elements from a different genre or style while keeping these core elements: {}. Genre shift for: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
            
            PivotStrategy::MetaResolution => {
                Some(format!(
                    "The logical complexity has reached a breaking point. Address this directly in the narrative - perhaps through a character realizing the situation is too complex, or through a narrative device that acknowledges the complexity and moves past it. Elements to maintain: {}. Context: {}",
                    self.format_elements_for_prompt(elements),
                    chapter_context
                ))
            },
        }
    }

    fn format_elements_for_prompt(&self, elements: &[ExtractedElement]) -> String {
        if elements.is_empty() {
            return "none extracted".to_string();
        }

        let mut formatted = String::new();
        let mut by_type: HashMap<String, Vec<&ExtractedElement>> = HashMap::new();
        
        for element in elements {
            let type_key = format!("{:?}", element.element_type);
            by_type.entry(type_key).or_insert_with(Vec::new).push(element);
        }

        for (element_type, element_list) in by_type {
            formatted.push_str(&format!("{}: ", element_type));
            let contents: Vec<&str> = element_list.iter().map(|e| e.content.as_str()).collect();
            formatted.push_str(&contents.join(", "));
            formatted.push_str("; ");
        }

        formatted.trim_end_matches("; ").to_string()
    }

    // Helper methods for pattern detection
    fn detect_stuck_patterns(&mut self, analysis: &ContentAnalysis) -> bool {
        // Implementation for detecting if AI is stuck in patterns
        let mut found_stuck = false;
        let analysis_content = analysis.content.clone();
        
        for pattern in &mut self.stuck_patterns {
            if Self::matches_stuck_pattern(&pattern, &analysis_content) {
                pattern.detection_count += 1;
                pattern.last_detected = Utc::now();
                if pattern.detection_count > 2 {
                    found_stuck = true;
                }
            }
        }
        found_stuck
    }

    fn matches_stuck_pattern(pattern: &StuckPattern, content: &str) -> bool {
        pattern.trigger_phrases.iter().any(|phrase| content.contains(phrase)) &&
        pattern.complexity_markers.iter().filter(|marker| content.contains(*marker)).count() > 2
    }

    fn calculate_pivot_confidence(&self, analysis: &ContentAnalysis) -> f32 {
        let issue_severity: f32 = analysis.confusion_indicators
            .iter()
            .map(|ci| ci.severity)
            .sum::<f32>() / analysis.confusion_indicators.len().max(1) as f32;
        
        (issue_severity * 0.7 + (1.0 - analysis.logical_coherence) * 0.3).min(1.0)
    }

    // Additional helper methods (simplified implementations)
    fn check_word_proximity(&self, content: &str, word1: &str, word2: &str) -> usize {
        // Simplified proximity check
        if let (Some(pos1), Some(pos2)) = (content.find(word1), content.find(word2)) {
            (pos1 as i32 - pos2 as i32).abs() as usize
        } else {
            1000 // Very far if not found
        }
    }

    fn extract_sample_around_words(&self, content: &str, word1: &str, word2: &str) -> String {
        // Extract text sample around the problematic words
        format!("...{}...{}", 
            content.get(content.find(word1).unwrap_or(0).saturating_sub(20)..content.find(word1).unwrap_or(0)+word1.len()+20)
                .unwrap_or(""),
            content.get(content.find(word2).unwrap_or(0).saturating_sub(20)..content.find(word2).unwrap_or(0)+word2.len()+20)
                .unwrap_or("")
        )
    }

    fn extract_sample_around_phrase(&self, content: &str, phrase: &str) -> String {
        if let Some(pos) = content.find(phrase) {
            let start = pos.saturating_sub(30);
            let end = (pos + phrase.len() + 30).min(content.len());
            format!("...{}...", &content[start..end])
        } else {
            phrase.to_string()
        }
    }

    fn extract_complexity_sample(&self, content: &str, markers: &[&str]) -> String {
        for marker in markers {
            if let Some(pos) = content.find(marker) {
                let start = pos.saturating_sub(20);
                let end = (pos + marker.len() + 20).min(content.len());
                return format!("...{}...", &content[start..end]);
            }
        }
        "Complex structure detected".to_string()
    }

    fn detect_temporal_conflicts(&self, content: &str, _time_indicators: &[&str]) -> f32 {
        // Simplified temporal conflict detection
        let past_tense = content.matches("was ").count() + content.matches("had ").count();
        let present_tense = content.matches("is ").count() + content.matches("are ").count();
        let future_tense = content.matches("will ").count() + content.matches("shall ").count();
        
        let total_tense = past_tense + present_tense + future_tense;
        if total_tense == 0 {
            return 0.0;
        }
        
        // If tenses are very mixed without clear transitions, it's confusing
        let tense_counts = [past_tense, present_tense, future_tense];
        let dominant_tense = *tense_counts.iter().max().unwrap();
        let tense_consistency = dominant_tense as f32 / total_tense as f32;
        
        if tense_consistency < 0.7 {
            1.0 - tense_consistency
        } else {
            0.0
        }
    }

    fn detect_pronoun_inconsistencies(&self, content: &str) -> f32 {
        // Simplified pronoun consistency check
        let pronouns = ["he", "she", "they", "it"];
        let mut pronoun_counts = HashMap::new();
        
        for pronoun in pronouns {
            let count = content.matches(&format!(" {} ", pronoun)).count();
            if count > 0 {
                *pronoun_counts.entry(pronoun).or_insert(0) += count;
            }
        }
        
        if pronoun_counts.len() > 2 {
            0.3 // Multiple pronouns might indicate confusion
        } else {
            0.0
        }
    }

    fn detect_logic_breaks(&self, content: &str) -> f32 {
        let contradiction_indicators = ["but", "however", "although", "despite", "nevertheless"];
        let total_contradictions = contradiction_indicators
            .iter()
            .map(|word| content.matches(word).count())
            .sum::<usize>();
        
        let word_count = content.split_whitespace().count();
        if word_count == 0 {
            return 0.0;
        }
        
        let contradiction_ratio = total_contradictions as f32 / word_count as f32;
        if contradiction_ratio > 0.05 { // More than 5% contradictions is problematic
            contradiction_ratio * 2.0
        } else {
            0.0
        }
    }

    pub fn get_recent_analysis_summary(&self) -> String {
        if self.recent_content_buffer.is_empty() {
            return "No recent analysis data".to_string();
        }

        let recent = &self.recent_content_buffer[self.recent_content_buffer.len() - 1];
        format!(
            "Latest Analysis: Complexity: {:.2}, Repetition: {:.2}, Readability: {:.2}, Momentum: {:.2}, Issues: {}",
            recent.sentence_complexity,
            recent.repetition_score,
            recent.readability_score,
            recent.narrative_momentum,
            recent.confusion_indicators.len()
        )
    }
}