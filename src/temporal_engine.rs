use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use crate::content::{Content, Section, Timeline, TimelineEvent, TemporalStructure};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEngine {
    pub narrative_timeline: NarrativeTimeline,
    pub temporal_context: TemporalContext,
    pub continuity_tracker: ContinuityTracker,
    pub pacing_controller: PacingController,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeTimeline {
    pub story_time: StoryTime,
    pub events: Vec<TemporalEvent>,
    pub time_jumps: Vec<TimeJump>,
    pub parallel_threads: HashMap<String, Vec<TemporalEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryTime {
    pub start_time: String,
    pub current_time: String,
    pub time_scale: TimeScale,
    pub duration_so_far: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeScale {
    Minutes,    // Real-time or near real-time
    Hours,      // Spans hours
    Days,       // Spans days
    Weeks,      // Spans weeks
    Months,     // Spans months
    Years,      // Spans years
    Decades,    // Epic timescales
    Flexible,   // Varies throughout story
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEvent {
    pub event_id: String,
    pub chapter_number: usize,
    pub section_number: usize,
    pub story_time: String,
    pub duration: String,
    pub event_type: TemporalEventType,
    pub description: String,
    pub consequences: Vec<String>,
    pub temporal_markers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalEventType {
    ActionSequence,
    DialogueScene,
    Transition,
    Flashback,
    FlashForward,
    Montage,
    Pause,          // Reflective moments
    Acceleration,   // Time passes quickly
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeJump {
    pub from_chapter: usize,
    pub to_chapter: usize,
    pub jump_type: TimeJumpType,
    pub duration: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeJumpType {
    Forward,        // Skip ahead in time
    Backward,       // Flashback
    Parallel,       // Switch to different character/location at same time
    Nested,         // Story within story
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub previous_events: Vec<String>,
    pub upcoming_events: Vec<String>,
    pub time_sensitive_elements: Vec<String>,
    pub temporal_anchors: HashMap<String, String>, // Key references to specific times
    pub chronological_constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityTracker {
    pub character_states: HashMap<String, CharacterTemporalState>,
    pub location_states: HashMap<String, LocationTemporalState>,
    pub plot_threads: HashMap<String, PlotThreadState>,
    pub inconsistencies: Vec<ContinuityIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTemporalState {
    pub last_seen: String,
    pub current_location: String,
    pub emotional_state: String,
    pub knowledge_state: Vec<String>,
    pub physical_state: String,
    pub temporal_arc: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTemporalState {
    pub current_time: String,
    pub weather_season: String,
    pub characters_present: Vec<String>,
    pub recent_events: Vec<String>,
    pub atmospheric_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotThreadState {
    pub current_stage: String,
    pub tension_level: f32,
    pub unresolved_elements: Vec<String>,
    pub approaching_climax: bool,
    pub temporal_urgency: UrgencyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Low,        // No time pressure
    Medium,     // Some time constraints
    High,       // Significant urgency
    Critical,   // Race against time
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityIssue {
    pub issue_type: ContinuityIssueType,
    pub description: String,
    pub affected_chapters: Vec<usize>,
    pub severity: IssueSeverity,
    pub suggested_fix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContinuityIssueType {
    TimeInconsistency,
    CharacterInconsistency,
    LocationInconsistency,
    PlotInconsistency,
    TemporalParadox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Minor,      // Small inconsistency
    Moderate,   // Noticeable issue
    Major,      // Significant problem
    Critical,   // Story-breaking error
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingController {
    pub chapter_pacing: Vec<ChapterPacing>,
    pub overall_rhythm: PacingPattern,
    pub tension_curve: Vec<f32>,
    pub scene_transitions: Vec<TransitionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterPacing {
    pub chapter_number: usize,
    pub pacing_type: PacingType,
    pub word_density: f32,      // Words per story time unit
    pub action_intensity: f32,  // 0.0 to 1.0
    pub emotional_weight: f32,  // 0.0 to 1.0
    pub temporal_focus: TemporalFocus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacingType {
    Slow,           // Contemplative, detailed
    Moderate,       // Balanced pacing
    Fast,           // Action-packed
    Variable,       // Changes within chapter
    Suspended,      // Time seems to stop
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalFocus {
    PastReflection,     // Looking back
    PresentAction,      // Current moment
    FutureAnticipation, // Looking ahead
    Timeless,           // Universal themes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacingPattern {
    Steady,         // Consistent throughout
    Escalating,     // Builds to climax
    Oscillating,    // Alternates fast/slow
    Episodic,       // Each chapter self-contained
    Spiral,         // Circular with progression
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Smooth,         // Seamless flow
    Cut,            // Abrupt change
    Fade,           // Gradual transition
    Bridge,         // Connecting element
    Parallel,       // Multiple simultaneous
}

impl TemporalEngine {
    pub fn new() -> Self {
        Self {
            narrative_timeline: NarrativeTimeline::new(),
            temporal_context: TemporalContext::new(),
            continuity_tracker: ContinuityTracker::new(),
            pacing_controller: PacingController::new(),
        }
    }

    pub fn analyze_chapter_context(&mut self, chapter_num: usize, content: &Content) -> Result<ChapterTemporalContext> {
        // Analyze what happened before
        let previous_context = self.build_previous_context(chapter_num)?;
        
        // Determine temporal requirements for this chapter
        let temporal_requirements = self.determine_temporal_requirements(chapter_num, content)?;
        
        // Check for continuity issues
        let continuity_issues = self.check_continuity(chapter_num, content)?;
        
        // Calculate optimal pacing
        let pacing_recommendation = self.calculate_optimal_pacing(chapter_num, content)?;

        Ok(ChapterTemporalContext {
            chapter_number: chapter_num,
            previous_context,
            temporal_requirements,
            continuity_issues,
            pacing_recommendation,
            time_anchors: self.get_relevant_time_anchors(chapter_num),
            character_states: self.get_current_character_states(),
            plot_urgencies: self.get_plot_urgencies(),
        })
    }

    pub fn generate_temporal_prompt_enhancement(&self, chapter_num: usize, base_prompt: &str, context: &ChapterTemporalContext) -> Result<String> {
        let mut enhanced_prompt = String::with_capacity(base_prompt.len() * 2);
        
        enhanced_prompt.push_str("=== TEMPORAL CONTEXT ===\n");
        enhanced_prompt.push_str(&format!("Chapter: {} | Story Time: {}\n", 
            chapter_num, context.temporal_requirements.current_time));
        
        if !context.previous_context.recent_events.is_empty() {
            enhanced_prompt.push_str("\nRECENT EVENTS:\n");
            for event in &context.previous_context.recent_events {
                enhanced_prompt.push_str(&format!("- {}\n", event));
            }
        }

        enhanced_prompt.push_str("\n=== CONTINUITY REQUIREMENTS ===\n");
        for req in &context.temporal_requirements.continuity_needs {
            enhanced_prompt.push_str(&format!("- {}\n", req));
        }

        enhanced_prompt.push_str(&format!("\n=== PACING GUIDANCE ===\n"));
        enhanced_prompt.push_str(&format!("Pacing: {:?} | Tension Level: {:.1}/10\n", 
            context.pacing_recommendation.pacing_type,
            context.pacing_recommendation.tension_level * 10.0));

        if !context.character_states.is_empty() {
            enhanced_prompt.push_str("\n=== CHARACTER STATES ===\n");
            for (character, state) in &context.character_states {
                enhanced_prompt.push_str(&format!("- {}: {} ({})\n", 
                    character, state.emotional_state, state.current_location));
            }
        }

        enhanced_prompt.push_str("\n=== ORIGINAL PROMPT ===\n");
        enhanced_prompt.push_str(base_prompt);

        Ok(enhanced_prompt)
    }

    pub fn update_after_chapter(&mut self, chapter_num: usize, generated_content: &str) -> Result<()> {
        // Extract temporal events from generated content
        let events = self.extract_temporal_events(chapter_num, generated_content)?;
        
        // Update narrative timeline
        for event in events {
            self.narrative_timeline.events.push(event);
        }

        // Update character states
        self.update_character_states(chapter_num, generated_content)?;

        // Update plot thread states
        self.update_plot_states(chapter_num, generated_content)?;

        // Check for new continuity issues
        let issues = self.detect_continuity_issues(chapter_num, generated_content)?;
        self.continuity_tracker.inconsistencies.extend(issues);

        // Update pacing analysis
        self.update_pacing_analysis(chapter_num, generated_content)?;

        Ok(())
    }

    fn build_previous_context(&self, chapter_num: usize) -> Result<PreviousContext> {
        let recent_events: Vec<String> = self.narrative_timeline.events
            .iter()
            .filter(|e| e.chapter_number >= chapter_num.saturating_sub(2) && e.chapter_number < chapter_num)
            .map(|e| format!("{}: {}", e.story_time, e.description))
            .collect();

        Ok(PreviousContext {
            recent_events,
            last_time_marker: self.get_latest_time_marker(),
            unresolved_threads: self.get_unresolved_plot_threads(),
        })
    }

    fn determine_temporal_requirements(&self, chapter_num: usize, content: &Content) -> Result<TemporalRequirements> {
        // This would analyze the content structure and determine what temporal elements are needed
        Ok(TemporalRequirements {
            current_time: format!("Chapter {} timeframe", chapter_num),
            time_passage: "Several hours".to_string(),
            continuity_needs: vec![
                "Maintain character emotional states".to_string(),
                "Progress main plot thread".to_string(),
            ],
            temporal_anchors_needed: vec![],
        })
    }

    fn check_continuity(&self, _chapter_num: usize, _content: &Content) -> Result<Vec<ContinuityIssue>> {
        // Check for temporal inconsistencies
        Ok(vec![])
    }

    fn calculate_optimal_pacing(&self, chapter_num: usize, _content: &Content) -> Result<PacingRecommendation> {
        let tension_level = if chapter_num <= 3 {
            0.3 // Building tension in early chapters
        } else if chapter_num <= 7 {
            0.6 // Rising action
        } else {
            0.8 // Climax approach
        };

        Ok(PacingRecommendation {
            pacing_type: PacingType::Moderate,
            tension_level,
            recommended_word_density: 1.2,
            scene_focus: TemporalFocus::PresentAction,
        })
    }

    fn get_relevant_time_anchors(&self, _chapter_num: usize) -> HashMap<String, String> {
        self.temporal_context.temporal_anchors.clone()
    }

    fn get_current_character_states(&self) -> HashMap<String, CharacterTemporalState> {
        self.continuity_tracker.character_states.clone()
    }

    fn get_plot_urgencies(&self) -> HashMap<String, UrgencyLevel> {
        self.continuity_tracker.plot_threads
            .iter()
            .map(|(k, v)| (k.clone(), v.temporal_urgency.clone()))
            .collect()
    }

    fn get_latest_time_marker(&self) -> String {
        self.narrative_timeline.story_time.current_time.clone()
    }

    fn get_unresolved_plot_threads(&self) -> Vec<String> {
        self.continuity_tracker.plot_threads
            .values()
            .flat_map(|thread| thread.unresolved_elements.iter())
            .cloned()
            .collect()
    }

    fn extract_temporal_events(&self, chapter_num: usize, content: &str) -> Result<Vec<TemporalEvent>> {
        // Analyze content for temporal markers and events
        // This is a simplified implementation
        Ok(vec![
            TemporalEvent {
                event_id: format!("ch{}_event1", chapter_num),
                chapter_number: chapter_num,
                section_number: 1,
                story_time: format!("Chapter {} time", chapter_num),
                duration: "1 hour".to_string(),
                event_type: TemporalEventType::ActionSequence,
                description: "Main chapter events".to_string(),
                consequences: vec![],
                temporal_markers: vec![],
            }
        ])
    }

    fn update_character_states(&mut self, _chapter_num: usize, _content: &str) -> Result<()> {
        // Update character states based on chapter content
        Ok(())
    }

    fn update_plot_states(&mut self, _chapter_num: usize, _content: &str) -> Result<()> {
        // Update plot thread progression
        Ok(())
    }

    fn detect_continuity_issues(&self, _chapter_num: usize, _content: &str) -> Result<Vec<ContinuityIssue>> {
        // Detect potential continuity problems
        Ok(vec![])
    }

    fn update_pacing_analysis(&mut self, chapter_num: usize, content: &str) -> Result<()> {
        let word_count = content.split_whitespace().count();
        let action_intensity = self.estimate_action_intensity(content);
        let emotional_weight = self.estimate_emotional_weight(content);

        let chapter_pacing = ChapterPacing {
            chapter_number: chapter_num,
            pacing_type: self.determine_pacing_type(action_intensity, emotional_weight),
            word_density: word_count as f32 / 1000.0, // Rough metric
            action_intensity,
            emotional_weight,
            temporal_focus: TemporalFocus::PresentAction,
        };

        if let Some(existing) = self.pacing_controller.chapter_pacing
            .iter_mut()
            .find(|cp| cp.chapter_number == chapter_num) {
            *existing = chapter_pacing;
        } else {
            self.pacing_controller.chapter_pacing.push(chapter_pacing);
        }

        Ok(())
    }

    fn estimate_action_intensity(&self, content: &str) -> f32 {
        let action_words = ["ran", "jumped", "fought", "attacked", "rushed", "grabbed", "struck"];
        let word_count = content.split_whitespace().count();
        let action_count = action_words.iter()
            .map(|word| content.matches(word).count())
            .sum::<usize>();

        (action_count as f32 / word_count as f32 * 100.0).min(1.0)
    }

    fn estimate_emotional_weight(&self, content: &str) -> f32 {
        let emotional_words = ["love", "hate", "fear", "joy", "anger", "sadness", "hope", "despair"];
        let word_count = content.split_whitespace().count();
        let emotional_count = emotional_words.iter()
            .map(|word| content.matches(word).count())
            .sum::<usize>();

        (emotional_count as f32 / word_count as f32 * 50.0).min(1.0)
    }

    fn determine_pacing_type(&self, action_intensity: f32, emotional_weight: f32) -> PacingType {
        if action_intensity > 0.7 {
            PacingType::Fast
        } else if emotional_weight > 0.6 {
            PacingType::Slow
        } else {
            PacingType::Moderate
        }
    }
}

// Helper structs for the engine
#[derive(Debug, Clone)]
pub struct ChapterTemporalContext {
    pub chapter_number: usize,
    pub previous_context: PreviousContext,
    pub temporal_requirements: TemporalRequirements,
    pub continuity_issues: Vec<ContinuityIssue>,
    pub pacing_recommendation: PacingRecommendation,
    pub time_anchors: HashMap<String, String>,
    pub character_states: HashMap<String, CharacterTemporalState>,
    pub plot_urgencies: HashMap<String, UrgencyLevel>,
}

#[derive(Debug, Clone)]
pub struct PreviousContext {
    pub recent_events: Vec<String>,
    pub last_time_marker: String,
    pub unresolved_threads: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TemporalRequirements {
    pub current_time: String,
    pub time_passage: String,
    pub continuity_needs: Vec<String>,
    pub temporal_anchors_needed: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PacingRecommendation {
    pub pacing_type: PacingType,
    pub tension_level: f32,
    pub recommended_word_density: f32,
    pub scene_focus: TemporalFocus,
}

// Implementation blocks for new structs
impl NarrativeTimeline {
    pub fn new() -> Self {
        Self {
            story_time: StoryTime::new(),
            events: Vec::new(),
            time_jumps: Vec::new(),
            parallel_threads: HashMap::new(),
        }
    }
}

impl StoryTime {
    pub fn new() -> Self {
        Self {
            start_time: "Beginning".to_string(),
            current_time: "Beginning".to_string(),
            time_scale: TimeScale::Hours,
            duration_so_far: "0 hours".to_string(),
        }
    }
}

impl TemporalContext {
    pub fn new() -> Self {
        Self {
            previous_events: Vec::new(),
            upcoming_events: Vec::new(),
            time_sensitive_elements: Vec::new(),
            temporal_anchors: HashMap::new(),
            chronological_constraints: Vec::new(),
        }
    }
}

impl ContinuityTracker {
    pub fn new() -> Self {
        Self {
            character_states: HashMap::new(),
            location_states: HashMap::new(),
            plot_threads: HashMap::new(),
            inconsistencies: Vec::new(),
        }
    }
}

impl PacingController {
    pub fn new() -> Self {
        Self {
            chapter_pacing: Vec::new(),
            overall_rhythm: PacingPattern::Escalating,
            tension_curve: Vec::new(),
            scene_transitions: Vec::new(),
        }
    }
}