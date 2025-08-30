use anyhow::Result;
use std::collections::HashMap;
use crate::cli_types::{Genre, WritingStyle};

/// The Silent Mind Writer - brings contemplative depth to writing through inner stillness
pub struct SilentMindWriter {
    contemplation_depth: f32,
    inner_stillness_level: f32,
    reflection_patterns: HashMap<String, ReflectionPattern>,
    current_meditation_state: MeditationState,
}

#[derive(Debug, Clone)]
pub struct ReflectionPattern {
    pub silence_duration: f32, // Metaphorical silence before writing
    pub contemplation_focus: ContemplationFocus,
    pub depth_multiplier: f32,
    pub inner_voice_quieting: f32, // How much to quiet the chatty inner voice
}

#[derive(Debug, Clone)]
pub enum ContemplationFocus {
    DeepListening,      // Listening to the story's deeper truth
    InnerWisdom,        // Accessing deeper knowledge
    SilentObservation,  // Observing without judgment
    QuietUnderstanding, // Understanding that comes from stillness
    MindfulAwareness,   // Aware presence in the writing
    StillPointCreativity, // Creating from the still point within
}

#[derive(Debug, Clone)]
pub enum MeditationState {
    Chattering,     // Inner voice is too active
    Settling,       // Beginning to quiet the mind
    Quieting,       // Mind becoming more still
    Silent,         // Deep inner silence
    Profound,       // Profound stillness and clarity
}

impl SilentMindWriter {
    pub fn new() -> Self {
        let mut reflection_patterns = HashMap::new();
        
        // Initialize reflection patterns for different scenarios
        Self::initialize_reflection_patterns(&mut reflection_patterns);
        
        Self {
            contemplation_depth: 0.5,
            inner_stillness_level: 0.3,
            reflection_patterns,
            current_meditation_state: MeditationState::Settling,
        }
    }
    
    fn initialize_reflection_patterns(patterns: &mut HashMap<String, ReflectionPattern>) {
        // Narrative writing - deep listening to the story
        patterns.insert("narrative_deep".to_string(), ReflectionPattern {
            silence_duration: 0.8,
            contemplation_focus: ContemplationFocus::DeepListening,
            depth_multiplier: 1.4,
            inner_voice_quieting: 0.7,
        });
        
        // Poetic writing - accessing inner wisdom
        patterns.insert("poetic_wisdom".to_string(), ReflectionPattern {
            silence_duration: 0.9,
            contemplation_focus: ContemplationFocus::InnerWisdom,
            depth_multiplier: 1.6,
            inner_voice_quieting: 0.8,
        });
        
        // Philosophical writing - silent observation
        patterns.insert("philosophical_observation".to_string(), ReflectionPattern {
            silence_duration: 0.85,
            contemplation_focus: ContemplationFocus::SilentObservation,
            depth_multiplier: 1.5,
            inner_voice_quieting: 0.75,
        });
        
        // Descriptive writing - mindful awareness
        patterns.insert("descriptive_mindful".to_string(), ReflectionPattern {
            silence_duration: 0.7,
            contemplation_focus: ContemplationFocus::MindfulAwareness,
            depth_multiplier: 1.3,
            inner_voice_quieting: 0.6,
        });
        
        // Creative writing - still point creativity
        patterns.insert("creative_stillpoint".to_string(), ReflectionPattern {
            silence_duration: 0.75,
            contemplation_focus: ContemplationFocus::StillPointCreativity,
            depth_multiplier: 1.4,
            inner_voice_quieting: 0.65,
        });
    }
    
    /// Enter a state of contemplative preparation before writing
    pub fn enter_contemplation(&mut self, genre: &Genre, style: &WritingStyle, content_type: &str) -> Result<ContemplativeState> {
        // Determine the appropriate reflection pattern
        let pattern_key = self.select_reflection_pattern(genre, style, content_type);
        let default_pattern = self.get_default_pattern();
        let pattern = self.reflection_patterns.get(&pattern_key)
            .unwrap_or(&default_pattern);
        
        // Clone the pattern to avoid borrowing issues
        let pattern_clone = pattern.clone();
        
        // Deepen the contemplation state
        self.deepen_stillness(&pattern_clone);
        
        // Create contemplative state for this writing session
        Ok(ContemplativeState {
            inner_silence_achieved: pattern_clone.inner_voice_quieting,
            contemplation_depth: pattern_clone.silence_duration,
            focus_area: pattern_clone.contemplation_focus.clone(),
            depth_enhancement: pattern_clone.depth_multiplier,
            meditation_guidance: self.generate_meditation_guidance(&pattern_clone),
        })
    }
    
    fn select_reflection_pattern(&self, genre: &Genre, style: &WritingStyle, content_type: &str) -> String {
        match (genre, style) {
            (Genre::Poetry, _) => "poetic_wisdom".to_string(),
            (Genre::Philosophy, _) | (Genre::Religion, _) => "philosophical_observation".to_string(),
            (_, WritingStyle::Poetic) | (_, WritingStyle::Descriptive) => "descriptive_mindful".to_string(),
            (_, WritingStyle::Creative) | (_, WritingStyle::Narrative) => "creative_stillpoint".to_string(),
            _ => {
                if content_type.contains("narrative") || content_type.contains("story") {
                    "narrative_deep".to_string()
                } else {
                    "creative_stillpoint".to_string()
                }
            }
        }
    }
    
    fn deepen_stillness(&mut self, pattern: &ReflectionPattern) {
        // Gradually quiet the inner chatter
        self.inner_stillness_level = (self.inner_stillness_level + pattern.inner_voice_quieting * 0.3).min(1.0);
        self.contemplation_depth = (self.contemplation_depth + pattern.silence_duration * 0.2).min(1.0);
        
        // Update meditation state based on stillness level
        self.current_meditation_state = match self.inner_stillness_level {
            x if x < 0.3 => MeditationState::Chattering,
            x if x < 0.5 => MeditationState::Settling,
            x if x < 0.7 => MeditationState::Quieting,
            x if x < 0.9 => MeditationState::Silent,
            _ => MeditationState::Profound,
        };
    }
    
    fn get_default_pattern(&self) -> ReflectionPattern {
        ReflectionPattern {
            silence_duration: 0.6,
            contemplation_focus: ContemplationFocus::QuietUnderstanding,
            depth_multiplier: 1.2,
            inner_voice_quieting: 0.5,
        }
    }
    
    fn generate_meditation_guidance(&self, pattern: &ReflectionPattern) -> String {
        match pattern.contemplation_focus {
            ContemplationFocus::DeepListening => 
                "Listen deeply to what wants to be written, beyond the surface thoughts".to_string(),
            ContemplationFocus::InnerWisdom => 
                "Access the wisdom that emerges from stillness and inner knowing".to_string(),
            ContemplationFocus::SilentObservation => 
                "Observe the subject with clear, unattached awareness".to_string(),
            ContemplationFocus::QuietUnderstanding => 
                "Allow understanding to arise naturally from quiet contemplation".to_string(),
            ContemplationFocus::MindfulAwareness => 
                "Write from a state of present-moment awareness and clarity".to_string(),
            ContemplationFocus::StillPointCreativity => 
                "Create from the still, creative center within".to_string(),
        }
    }
    
    /// Generate enhanced writing prompts that encourage contemplative depth
    pub fn create_contemplative_prompt(&self, base_prompt: &str, contemplative_state: &ContemplativeState) -> String {
        let silence_prefix = self.generate_silence_prefix(&contemplative_state.focus_area);
        let depth_enhancement = self.generate_depth_enhancement(contemplative_state);
        let stillness_guidance = self.generate_stillness_guidance();
        
        format!(
            "{}\n\n{}\n\n{}\n\n{}\n\nNow, from this state of inner stillness and contemplative awareness, {}",
            silence_prefix,
            depth_enhancement,
            stillness_guidance,
            contemplative_state.meditation_guidance,
            base_prompt
        )
    }
    
    fn generate_silence_prefix(&self, focus: &ContemplationFocus) -> String {
        match focus {
            ContemplationFocus::DeepListening => 
                "Before writing, rest in deep listening silence. Let the inner chatter settle like sediment in still water. What emerges from this quiet listening?".to_string(),
            ContemplationFocus::InnerWisdom => 
                "Enter the sanctuary of inner silence where wisdom dwells. Beyond the busy mind lies a knowing that needs no words to understand itself.".to_string(),
            ContemplationFocus::SilentObservation => 
                "From a place of witness consciousness, observe without the commentary of thought. What is seen when the mind's chatter ceases?".to_string(),
            ContemplationFocus::QuietUnderstanding => 
                "In the quiet space between thoughts, understanding blooms like a flower opening to sunlight. Rest here before writing.".to_string(),
            ContemplationFocus::MindfulAwareness => 
                "Ground yourself in present-moment awareness. Feel the stillness beneath all mental activity. From this centered place, what wants expression?".to_string(),
            ContemplationFocus::StillPointCreativity => 
                "Touch the still point at the center of your being - the source from which all creativity flows. Here, inspiration and wisdom are one.".to_string(),
        }
    }
    
    fn generate_depth_enhancement(&self, state: &ContemplativeState) -> String {
        let depth_level = (state.depth_enhancement * 100.0) as u32;
        
        match depth_level {
            0..=120 => "Write with gentle depth, allowing thoughts to settle naturally.".to_string(),
            121..=140 => "Draw from deeper wells of understanding, beyond surface thoughts.".to_string(),
            141..=160 => "Access profound layers of meaning that emerge from sustained contemplation.".to_string(),
            _ => "Write from the deepest silence, where words carry the weight of true understanding.".to_string(),
        }
    }
    
    fn generate_stillness_guidance(&self) -> String {
        match self.current_meditation_state {
            MeditationState::Chattering => 
                "Notice the inner chatter, but don't be carried away by it. Let thoughts pass like clouds in an open sky.".to_string(),
            MeditationState::Settling => 
                "Feel the mind beginning to settle. Like a lake becoming calm, clarity emerges naturally.".to_string(),
            MeditationState::Quieting => 
                "Rest in the growing quietude. The space between thoughts becomes more apparent and spacious.".to_string(),
            MeditationState::Silent => 
                "From this inner silence, words arise with natural authority and depth. Trust what emerges.".to_string(),
            MeditationState::Profound => 
                "Write from the profound stillness that underlies all expression. Here, every word carries the essence of understanding.".to_string(),
        }
    }
    
    /// Process generated content to remove excessive chattiness
    pub fn refine_contemplative_output(&self, raw_content: &str, contemplative_state: &ContemplativeState) -> String {
        let lines: Vec<&str> = raw_content.lines().collect();
        let refined_lines: Vec<String> = lines.iter()
            .map(|line| self.apply_contemplative_refinement(line, contemplative_state))
            .filter(|line| !line.trim().is_empty())
            .collect();
        
        refined_lines.join("\n")
    }
    
    fn apply_contemplative_refinement(&self, line: &str, state: &ContemplativeState) -> String {
        let mut refined = line.to_string();
        
        // Remove excessive conversational markers
        refined = self.reduce_chattiness(&refined, state.inner_silence_achieved);
        
        // Enhance depth and contemplative quality
        refined = self.enhance_contemplative_depth(&refined, state.depth_enhancement);
        
        // Apply stillness-based improvements
        refined = self.apply_stillness_refinements(&refined);
        
        refined
    }
    
    fn reduce_chattiness(&self, text: &str, silence_level: f32) -> String {
        let mut result = text.to_string();
        
        if silence_level > 0.6 {
            // Remove excessive "well", "you know", "actually", etc.
            let chatty_words = ["well, ", "you know, ", "actually, ", "I mean, ", "like, ", "basically, "];
            for word in &chatty_words {
                result = result.replace(word, "");
            }
            
            // Replace conversational transitions with more contemplative ones
            result = result.replace("And then, ", "");
            result = result.replace("So, ", "");
            result = result.replace("Anyway, ", "");
        }
        
        if silence_level > 0.8 {
            // Remove redundant explanatory phrases
            result = result.replace("It's important to note that ", "");
            result = result.replace("What I'm trying to say is ", "");
            result = result.replace("The point is that ", "");
        }
        
        result
    }
    
    fn enhance_contemplative_depth(&self, text: &str, depth_multiplier: f32) -> String {
        if depth_multiplier < 1.3 {
            return text.to_string();
        }
        
        // Add contemplative pauses (represented as thoughtful phrasing)
        let mut result = text.to_string();
        
        // Replace rushed connections with more thoughtful ones
        result = result.replace(" and ", "... and ");
        result = result.replace(", but ", "... yet ");
        result = result.replace(" because ", "... for ");
        
        // Add contemplative depth to observations
        if result.contains("is") && depth_multiplier > 1.4 {
            result = result.replace(" is ", " dwells as ");
            result = result.replace(" are ", " exist as ");
        }
        
        result
    }
    
    fn apply_stillness_refinements(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Replace urgent language with more contemplative phrasing
        result = result.replace("immediately", "in due time");
        result = result.replace("quickly", "gently");
        result = result.replace("suddenly", "gradually");
        result = result.replace("must", "may");
        result = result.replace("should", "might");
        
        // Enhance with contemplative language where appropriate
        if result.len() > 50 { // Only for substantial text
            result = result.replace(" understand ", " perceive ");
            result = result.replace(" think ", " contemplate ");
            result = result.replace(" know ", " sense ");
        }
        
        result
    }
    
    /// Get current meditation state description
    pub fn get_meditation_state_description(&self) -> String {
        match self.current_meditation_state {
            MeditationState::Chattering => "Inner voice is active and chattery".to_string(),
            MeditationState::Settling => "Mind is beginning to settle into quietude".to_string(),
            MeditationState::Quieting => "Thoughts are becoming more spacious and clear".to_string(),
            MeditationState::Silent => "Deep inner silence has been achieved".to_string(),
            MeditationState::Profound => "Profound stillness underlies all expression".to_string(),
        }
    }
    
    /// Reset the meditation state for a new session
    pub fn reset_contemplation(&mut self) {
        self.contemplation_depth = 0.5;
        self.inner_stillness_level = 0.3;
        self.current_meditation_state = MeditationState::Settling;
    }
}

#[derive(Debug, Clone)]
pub struct ContemplativeState {
    pub inner_silence_achieved: f32,
    pub contemplation_depth: f32,
    pub focus_area: ContemplationFocus,
    pub depth_enhancement: f32,
    pub meditation_guidance: String,
}

impl Default for SilentMindWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_silent_mind_initialization() {
        let writer = SilentMindWriter::new();
        assert!(writer.contemplation_depth > 0.0);
        assert!(writer.inner_stillness_level > 0.0);
        assert!(!writer.reflection_patterns.is_empty());
    }
    
    #[test]
    fn test_contemplation_entrance() {
        let mut writer = SilentMindWriter::new();
        let result = writer.enter_contemplation(&Genre::Poetry, &WritingStyle::Poetic, "poetry");
        assert!(result.is_ok());
        
        let state = result.unwrap();
        assert!(state.inner_silence_achieved > 0.0);
        assert!(state.depth_enhancement > 1.0);
    }
    
    #[test]
    fn test_chattiness_reduction() {
        let writer = SilentMindWriter::new();
        let chatty_text = "Well, you know, I think that we should actually consider this approach.";
        let refined = writer.reduce_chattiness(chatty_text, 0.8);
        assert!(!refined.contains("well"));
        assert!(!refined.contains("you know"));
        assert!(!refined.contains("actually"));
    }
}