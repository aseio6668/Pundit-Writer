// Dynamic section length system for creative and adaptive content generation
use crate::content::{ContentType, SectionType};
use serde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicLengthConfig {
    pub content_type: ContentType,
    pub use_dynamic_lengths: bool,
    pub base_target_words: usize,
    pub variation_range: (f32, f32), // (min_multiplier, max_multiplier)
    pub length_patterns: Vec<LengthPattern>,
    pub narrative_flow_weights: NarrativeFlowWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LengthPattern {
    pub pattern_name: String,
    pub section_weights: Vec<f32>, // Relative weights for each section in pattern
    pub repeatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeFlowWeights {
    pub opening_weight: f32,     // First sections tend to be longer/shorter
    pub middle_weight: f32,      // Middle sections weight
    pub climax_weight: f32,      // Climactic sections weight  
    pub resolution_weight: f32,  // Final sections weight
    pub transition_weight: f32,  // Transition sections weight
}

#[derive(Debug, Clone)]
pub struct DynamicSectionLength {
    pub section_number: usize,
    pub target_words: usize,
    pub min_words: usize,
    pub max_words: usize,
    pub priority_level: PriorityLevel,
    pub narrative_purpose: NarrativePurpose,
}

#[derive(Debug, Clone)]
pub enum PriorityLevel {
    Critical,    // Key plot points, must be longer
    Important,   // Character development, substantial content
    Standard,    // Normal narrative flow
    Brief,       // Transitions, quick scenes
    Flexible,    // Can be any length based on need
}

#[derive(Debug, Clone)]
pub enum NarrativePurpose {
    Opening,        // Introduction, setup
    Development,    // Character/plot development  
    Action,         // Action sequences, fast-paced
    Dialogue,       // Conversation-heavy sections
    Description,    // World-building, setting
    Transition,     // Moving between scenes/locations
    Climax,         // Major dramatic moments
    Resolution,     // Conclusions, wrap-up
    Reflection,     // Introspective moments
    Exposition,     // Information delivery
}

impl DynamicLengthConfig {
    pub fn for_content_type(content_type: ContentType) -> Self {
        match content_type {
            ContentType::Book => Self::for_books(),
            ContentType::Poetry => Self::for_poetry(),
            ContentType::Screenplay => Self::for_screenplays(),
            ContentType::Play => Self::for_plays(),
            ContentType::TvScript => Self::for_tv_scripts(),
            ContentType::AudioScript => Self::for_audio_scripts(),
            ContentType::GameScript => Self::for_game_scripts(),
            ContentType::TechnicalDoc => Self::for_technical_docs(),
            ContentType::ResearchReport => Self::for_research_reports(),
            ContentType::MarketingAd => Self::for_marketing_content(),
            ContentType::BlogPost => Self::for_blog_posts(),
            ContentType::Document => Self::for_business_documents(),
            _ => Self::default_fixed_length(content_type),
        }
    }

    fn for_books() -> Self {
        Self {
            content_type: ContentType::Book,
            use_dynamic_lengths: true,
            base_target_words: 3000,
            variation_range: (0.4, 2.5), // 1,200 to 7,500 words
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Classic Arc".to_string(),
                    section_weights: vec![1.2, 1.0, 0.8, 1.1, 1.4, 1.0, 0.9, 1.6, 1.3, 0.8],
                    repeatable: true,
                },
                LengthPattern {
                    pattern_name: "Action Heavy".to_string(),
                    section_weights: vec![1.0, 0.6, 0.7, 0.5, 1.2, 0.6, 0.8, 1.8, 0.7, 1.0],
                    repeatable: true,
                },
                LengthPattern {
                    pattern_name: "Character Study".to_string(),
                    section_weights: vec![1.5, 1.3, 1.2, 1.4, 1.1, 1.3, 1.2, 1.4, 1.6, 1.2],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.3,
                middle_weight: 1.0,
                climax_weight: 1.8,
                resolution_weight: 1.1,
                transition_weight: 0.6,
            },
        }
    }

    fn for_poetry() -> Self {
        Self {
            content_type: ContentType::Poetry,
            use_dynamic_lengths: true,
            base_target_words: 150,
            variation_range: (0.2, 10.0), // 30 to 1,500 words (haiku to epic)
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Mixed Forms".to_string(),
                    section_weights: vec![0.2, 1.0, 0.5, 2.0, 0.3, 1.5, 0.4, 3.0],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.2,
                middle_weight: 1.0,
                climax_weight: 2.5,
                resolution_weight: 0.8,
                transition_weight: 0.3,
            },
        }
    }

    fn for_screenplays() -> Self {
        Self {
            content_type: ContentType::Screenplay,
            use_dynamic_lengths: true,
            base_target_words: 1200, // ~5 pages
            variation_range: (0.3, 3.0), // 360 to 3,600 words
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Film Structure".to_string(),
                    section_weights: vec![1.5, 0.8, 1.0, 0.6, 1.2, 0.7, 2.0, 1.1, 0.9],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.4,
                middle_weight: 1.0,
                climax_weight: 2.2,
                resolution_weight: 1.1,
                transition_weight: 0.5,
            },
        }
    }

    fn for_plays() -> Self {
        Self {
            content_type: ContentType::Play,
            use_dynamic_lengths: true,
            base_target_words: 1500,
            variation_range: (0.4, 2.5),
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Three Act".to_string(),
                    section_weights: vec![1.2, 1.0, 1.1, 0.8, 1.6, 1.2, 0.9, 1.4, 1.0],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.3,
                middle_weight: 1.0,
                climax_weight: 1.9,
                resolution_weight: 1.2,
                transition_weight: 0.7,
            },
        }
    }

    fn for_tv_scripts() -> Self {
        Self {
            content_type: ContentType::TvScript,
            use_dynamic_lengths: true,
            base_target_words: 800,
            variation_range: (0.5, 2.0),
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "TV Episode".to_string(),
                    section_weights: vec![1.1, 0.9, 1.0, 0.7, 1.3, 0.8, 1.1, 1.2],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.2,
                middle_weight: 1.0,
                climax_weight: 1.5,
                resolution_weight: 1.0,
                transition_weight: 0.6,
            },
        }
    }

    fn for_audio_scripts() -> Self {
        Self {
            content_type: ContentType::AudioScript,
            use_dynamic_lengths: true,
            base_target_words: 600,
            variation_range: (0.4, 2.5),
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Audio Flow".to_string(),
                    section_weights: vec![1.0, 0.8, 1.2, 0.6, 1.4, 0.9, 1.1],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.1,
                middle_weight: 1.0,
                climax_weight: 1.6,
                resolution_weight: 0.9,
                transition_weight: 0.5,
            },
        }
    }

    fn for_game_scripts() -> Self {
        Self {
            content_type: ContentType::GameScript,
            use_dynamic_lengths: true,
            base_target_words: 400,
            variation_range: (0.3, 4.0), // Small interactions to major cutscenes
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Game Interaction".to_string(),
                    section_weights: vec![0.5, 1.2, 0.4, 2.0, 0.6, 1.5, 0.3, 1.8],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.5,
                middle_weight: 1.0,
                climax_weight: 2.5,
                resolution_weight: 1.2,
                transition_weight: 0.3,
            },
        }
    }

    fn for_technical_docs() -> Self {
        Self {
            content_type: ContentType::TechnicalDoc,
            use_dynamic_lengths: true,
            base_target_words: 2000,
            variation_range: (0.7, 1.5), // Limited variation for readability
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Technical Flow".to_string(),
                    section_weights: vec![1.2, 1.0, 1.1, 0.9, 1.0, 1.1, 0.8, 1.0],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.2,
                middle_weight: 1.0,
                climax_weight: 1.1,
                resolution_weight: 1.0,
                transition_weight: 0.8,
            },
        }
    }

    fn for_research_reports() -> Self {
        Self {
            content_type: ContentType::ResearchReport,
            use_dynamic_lengths: true,
            base_target_words: 2500,
            variation_range: (0.6, 1.8),
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Research Structure".to_string(),
                    section_weights: vec![1.3, 1.0, 1.2, 1.1, 1.4, 1.0, 0.8, 1.1],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.3,
                middle_weight: 1.1,
                climax_weight: 1.4,
                resolution_weight: 1.0,
                transition_weight: 0.8,
            },
        }
    }

    fn for_marketing_content() -> Self {
        Self {
            content_type: ContentType::MarketingAd,
            use_dynamic_lengths: true,
            base_target_words: 800,
            variation_range: (0.3, 2.0),
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Marketing Flow".to_string(),
                    section_weights: vec![1.2, 0.6, 1.5, 0.8, 1.0, 1.3, 0.7],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.3,
                middle_weight: 1.0,
                climax_weight: 1.6,
                resolution_weight: 1.2,
                transition_weight: 0.6,
            },
        }
    }

    fn for_blog_posts() -> Self {
        Self {
            content_type: ContentType::BlogPost,
            use_dynamic_lengths: true,
            base_target_words: 1200,
            variation_range: (0.5, 2.0),
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Blog Structure".to_string(),
                    section_weights: vec![1.1, 1.0, 1.2, 0.8, 1.1, 1.0, 0.9, 1.1],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.2,
                middle_weight: 1.0,
                climax_weight: 1.3,
                resolution_weight: 1.1,
                transition_weight: 0.8,
            },
        }
    }

    fn for_business_documents() -> Self {
        Self {
            content_type: ContentType::Document,
            use_dynamic_lengths: false, // Keep structured for professionalism
            base_target_words: 2000,
            variation_range: (0.8, 1.2), // Very limited variation
            length_patterns: vec![
                LengthPattern {
                    pattern_name: "Business Structure".to_string(),
                    section_weights: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                    repeatable: true,
                },
            ],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.1,
                middle_weight: 1.0,
                climax_weight: 1.0,
                resolution_weight: 1.0,
                transition_weight: 1.0,
            },
        }
    }

    fn default_fixed_length(content_type: ContentType) -> Self {
        Self {
            content_type,
            use_dynamic_lengths: false,
            base_target_words: 2000,
            variation_range: (1.0, 1.0),
            length_patterns: vec![],
            narrative_flow_weights: NarrativeFlowWeights {
                opening_weight: 1.0,
                middle_weight: 1.0,
                climax_weight: 1.0,
                resolution_weight: 1.0,
                transition_weight: 1.0,
            },
        }
    }
}

impl DynamicSectionLength {
    pub fn calculate_for_section(
        config: &DynamicLengthConfig,
        section_number: usize,
        total_sections: usize,
        seed: Option<u64>,
    ) -> Self {
        if !config.use_dynamic_lengths {
            return Self::fixed_length(section_number, config.base_target_words);
        }

        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };

        // Determine narrative purpose based on position
        let narrative_purpose = Self::determine_narrative_purpose(section_number, total_sections);
        
        // Get narrative flow weight
        let flow_weight = Self::get_narrative_flow_weight(config, section_number, total_sections);
        
        // Apply pattern-based variation
        let pattern_weight = Self::get_pattern_weight(config, section_number, &mut rng);
        
        // Apply random variation within bounds
        let random_factor = rng.gen_range(config.variation_range.0..=config.variation_range.1);
        
        // Combine all factors
        let combined_weight = flow_weight * pattern_weight * random_factor;
        let target_words = (config.base_target_words as f32 * combined_weight) as usize;
        
        // Calculate bounds (±30% of target)
        let min_words = (target_words as f32 * 0.7) as usize;
        let max_words = (target_words as f32 * 1.3) as usize;
        
        // Determine priority level
        let priority_level = Self::determine_priority_level(&narrative_purpose, combined_weight);

        Self {
            section_number,
            target_words,
            min_words,
            max_words,
            priority_level,
            narrative_purpose,
        }
    }

    fn fixed_length(section_number: usize, target_words: usize) -> Self {
        Self {
            section_number,
            target_words,
            min_words: target_words,
            max_words: target_words,
            priority_level: PriorityLevel::Standard,
            narrative_purpose: NarrativePurpose::Development,
        }
    }

    fn determine_narrative_purpose(section_number: usize, total_sections: usize) -> NarrativePurpose {
        let position_ratio = section_number as f32 / total_sections as f32;
        
        match position_ratio {
            p if p <= 0.1 => NarrativePurpose::Opening,
            p if p <= 0.3 => NarrativePurpose::Development,
            p if p <= 0.7 => {
                // Vary between different purposes in middle
                match section_number % 4 {
                    0 => NarrativePurpose::Action,
                    1 => NarrativePurpose::Dialogue,
                    2 => NarrativePurpose::Description,
                    _ => NarrativePurpose::Development,
                }
            },
            p if p <= 0.85 => NarrativePurpose::Climax,
            _ => NarrativePurpose::Resolution,
        }
    }

    fn get_narrative_flow_weight(
        config: &DynamicLengthConfig,
        section_number: usize,
        total_sections: usize,
    ) -> f32 {
        let position_ratio = section_number as f32 / total_sections as f32;
        
        match position_ratio {
            p if p <= 0.2 => config.narrative_flow_weights.opening_weight,
            p if p <= 0.7 => config.narrative_flow_weights.middle_weight,
            p if p <= 0.85 => config.narrative_flow_weights.climax_weight,
            _ => config.narrative_flow_weights.resolution_weight,
        }
    }

    fn get_pattern_weight(
        config: &DynamicLengthConfig,
        section_number: usize,
        rng: &mut StdRng,
    ) -> f32 {
        if config.length_patterns.is_empty() {
            return 1.0;
        }

        // Choose a random pattern
        let pattern_idx = rng.gen_range(0..config.length_patterns.len());
        let pattern = &config.length_patterns[pattern_idx];
        
        if pattern.section_weights.is_empty() {
            return 1.0;
        }

        // Get weight from pattern (cycling if necessary)
        let weight_idx = (section_number - 1) % pattern.section_weights.len();
        pattern.section_weights[weight_idx]
    }

    fn determine_priority_level(purpose: &NarrativePurpose, weight: f32) -> PriorityLevel {
        match purpose {
            NarrativePurpose::Opening | NarrativePurpose::Climax | NarrativePurpose::Resolution => {
                if weight >= 1.5 {
                    PriorityLevel::Critical
                } else {
                    PriorityLevel::Important
                }
            },
            NarrativePurpose::Transition => PriorityLevel::Brief,
            NarrativePurpose::Action if weight <= 0.7 => PriorityLevel::Brief,
            _ => PriorityLevel::Standard,
        }
    }

    pub fn get_generation_prompt_addition(&self) -> String {
        let length_guidance = match self.priority_level {
            PriorityLevel::Critical => "This is a critical section that requires substantial depth and detail.",
            PriorityLevel::Important => "This is an important section that should be well-developed.",
            PriorityLevel::Standard => "This section should have appropriate depth for the narrative.",
            PriorityLevel::Brief => "This should be a concise, efficient section.",
            PriorityLevel::Flexible => "Adjust the length based on what the content naturally requires.",
        };

        let purpose_guidance = match self.narrative_purpose {
            NarrativePurpose::Opening => "Focus on engaging the reader and establishing the foundation.",
            NarrativePurpose::Development => "Develop characters, plot, or concepts in meaningful ways.",
            NarrativePurpose::Action => "Keep the pacing brisk and dynamic.",
            NarrativePurpose::Dialogue => "Focus on character interaction and conversation.",
            NarrativePurpose::Description => "Provide rich, immersive detail and atmosphere.",
            NarrativePurpose::Transition => "Efficiently move the narrative forward.",
            NarrativePurpose::Climax => "Build tension and deliver significant dramatic impact.",
            NarrativePurpose::Resolution => "Provide satisfying closure and conclusions.",
            NarrativePurpose::Reflection => "Allow for introspection and emotional depth.",
            NarrativePurpose::Exposition => "Clearly convey necessary information.",
        };

        format!(
            "\n\nLENGTH GUIDANCE:\n- Target: {} words (range: {}-{} words)\n- {}\n- {}\n",
            self.target_words,
            self.min_words,
            self.max_words,
            length_guidance,
            purpose_guidance
        )
    }
}

pub fn generate_dynamic_section_lengths(
    content_type: ContentType,
    total_sections: usize,
    seed: Option<u64>,
) -> Vec<DynamicSectionLength> {
    let config = DynamicLengthConfig::for_content_type(content_type);
    
    (1..=total_sections)
        .map(|section_num| {
            DynamicSectionLength::calculate_for_section(
                &config,
                section_num,
                total_sections,
                seed,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_dynamic_lengths() {
        let lengths = generate_dynamic_section_lengths(ContentType::Book, 10, Some(42));
        
        // Should have variation
        let word_counts: Vec<usize> = lengths.iter().map(|l| l.target_words).collect();
        let min_words = *word_counts.iter().min().unwrap();
        let max_words = *word_counts.iter().max().unwrap();
        
        // Should have significant variation for books
        assert!((max_words as f32) > (min_words as f32 * 1.5));
        
        // All should be within reasonable bounds
        for length in &lengths {
            assert!(length.target_words >= 500);
            assert!(length.target_words <= 10000);
        }
    }

    #[test]
    fn test_business_doc_consistency() {
        let lengths = generate_dynamic_section_lengths(ContentType::Document, 6, Some(42));
        
        // Business docs should have minimal variation
        let word_counts: Vec<usize> = lengths.iter().map(|l| l.target_words).collect();
        let min_words = *word_counts.iter().min().unwrap();
        let max_words = *word_counts.iter().max().unwrap();
        
        // Should have minimal variation for business docs
        assert!((max_words as f32) < (min_words as f32 * 1.3));
    }

    #[test]
    fn test_poetry_extreme_variation() {
        let lengths = generate_dynamic_section_lengths(ContentType::Poetry, 8, Some(42));
        
        // Poetry should have extreme variation
        let word_counts: Vec<usize> = lengths.iter().map(|l| l.target_words).collect();
        let min_words = *word_counts.iter().min().unwrap();
        let max_words = *word_counts.iter().max().unwrap();
        
        // Should have very high variation for poetry
        assert!((max_words as f32) > (min_words as f32 * 3.0));
    }
}