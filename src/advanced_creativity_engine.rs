use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::cli_types::{Genre, WritingStyle};
use crate::content::ContentType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCreativityEngine {
    pub creativity_profile: CreativityProfile,
    pub narrative_dna: NarrativeDNA,
    pub creative_constraints: CreativeConstraints,
    pub inspiration_sources: Vec<InspirationSource>,
    pub adaptive_parameters: AdaptiveParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityProfile {
    pub originality_level: f32,        // 0.0 to 1.0
    pub complexity_preference: f32,    // 0.0 to 1.0  
    pub risk_tolerance: f32,           // 0.0 to 1.0
    pub stylistic_consistency: f32,    // 0.0 to 1.0
    pub thematic_depth: f32,          // 0.0 to 1.0
    pub character_focus: f32,         // 0.0 to 1.0
    pub world_building_intensity: f32, // 0.0 to 1.0
    pub emotional_range: f32,         // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeDNA {
    pub core_themes: Vec<ThematicElement>,
    pub character_archetypes: Vec<ArchetypeTemplate>,
    pub plot_patterns: Vec<PlotPattern>,
    pub stylistic_markers: Vec<StylisticMarker>,
    pub world_building_elements: Vec<WorldElement>,
    pub conflict_types: Vec<ConflictArchetype>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThematicElement {
    pub theme: String,
    pub intensity: f32,
    pub manifestation_methods: Vec<String>,
    pub symbolic_representations: Vec<String>,
    pub character_connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeTemplate {
    pub archetype_name: String,
    pub core_traits: Vec<String>,
    pub motivations: Vec<String>,
    pub conflict_generators: Vec<String>,
    pub growth_arcs: Vec<String>,
    pub dialogue_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotPattern {
    pub pattern_name: String,
    pub structure: PlotStructure,
    pub tension_curve: Vec<f32>,
    pub key_beats: Vec<PlotBeat>,
    pub variation_possibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotStructure {
    ThreeAct,
    HeroJourney,
    Freytag,
    Kishōtenketsu,   // Japanese 4-act structure
    Spiral,
    Parallel,
    NonLinear,
    Experimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotBeat {
    pub beat_name: String,
    pub typical_position: f32,  // 0.0 to 1.0 through story
    pub function: String,
    pub emotional_impact: f32,
    pub creative_opportunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylisticMarker {
    pub style_element: String,
    pub application_frequency: f32,
    pub context_rules: Vec<String>,
    pub variations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldElement {
    pub element_type: WorldElementType,
    pub description: String,
    pub creative_potential: f32,
    pub interconnections: Vec<String>,
    pub expansion_possibilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldElementType {
    Setting,
    Culture,
    Technology,
    Magic,
    History,
    Geography,
    Society,
    Economy,
    Politics,
    Religion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictArchetype {
    pub conflict_name: String,
    pub conflict_type: ConflictType,
    pub escalation_patterns: Vec<String>,
    pub resolution_methods: Vec<String>,
    pub character_impact: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    PersonVsPerson,
    PersonVsSelf,
    PersonVsSociety,
    PersonVsNature,
    PersonVsTechnology,
    PersonVsSupernatural,
    PersonVsFate,
    IdeologicalClash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeConstraints {
    pub genre_expectations: Vec<String>,
    pub style_requirements: Vec<String>,
    pub content_boundaries: Vec<String>,
    pub target_audience_considerations: Vec<String>,
    pub thematic_limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspirationSource {
    pub source_type: InspirationSourceType,
    pub content: String,
    pub creative_potential: f32,
    pub application_contexts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InspirationSourceType {
    Historical,
    Mythological,
    Scientific,
    Philosophical,
    Artistic,
    Literary,
    Cultural,
    Personal,
    Observational,
    Speculative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveParameters {
    pub chapter_specific_adjustments: HashMap<usize, CreativityAdjustment>,
    pub dynamic_tension_management: TensionManager,
    pub reader_engagement_optimization: EngagementOptimizer,
    pub narrative_coherence_maintainer: CoherenceMaintainer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityAdjustment {
    pub originality_modifier: f32,
    pub complexity_modifier: f32,
    pub focus_shifts: Vec<String>,
    pub special_techniques: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensionManager {
    pub target_tension_curve: Vec<f32>,
    pub tension_building_techniques: Vec<String>,
    pub release_mechanisms: Vec<String>,
    pub pacing_adjustments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementOptimizer {
    pub hook_strategies: Vec<String>,
    pub curiosity_generators: Vec<String>,
    pub emotional_connection_methods: Vec<String>,
    pub surprise_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceMaintainer {
    pub consistency_rules: Vec<String>,
    pub theme_reinforcement: Vec<String>,
    pub character_arc_tracking: Vec<String>,
    pub plot_thread_management: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeChapterPlan {
    pub chapter_number: usize,
    pub creative_focus: Vec<CreativeFocus>,
    pub innovative_techniques: Vec<InnovativeTechnique>,
    pub thematic_development: Vec<String>,
    pub character_development_opportunities: Vec<String>,
    pub plot_advancement_strategies: Vec<String>,
    pub stylistic_experiments: Vec<String>,
    pub reader_surprise_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreativeFocus {
    CharacterDepth,
    WorldExpansion,
    ThematicExploration,
    PlotComplexity,
    StylisticInnovation,
    EmotionalResonance,
    IntellectualEngagement,
    SensoryExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnovativeTechnique {
    pub technique_name: String,
    pub description: String,
    pub application_method: String,
    pub creative_impact: f32,
    pub risk_level: f32,
    pub genre_compatibility: Vec<Genre>,
}

impl AdvancedCreativityEngine {
    pub fn new(genre: &Genre, style: &WritingStyle, content_type: &ContentType) -> Self {
        let mut engine = Self {
            creativity_profile: CreativityProfile::for_genre_style(genre, style),
            narrative_dna: NarrativeDNA::for_genre(genre),
            creative_constraints: CreativeConstraints::for_content_type(content_type),
            inspiration_sources: Self::generate_inspiration_sources(genre, style),
            adaptive_parameters: AdaptiveParameters::new(),
        };

        engine.calibrate_for_genre_style(genre, style);
        engine
    }

    pub fn generate_creative_chapter_plan(&mut self, chapter_number: usize, story_context: &str, interruption_recovery: bool) -> Result<CreativeChapterPlan> {
        let mut rng = StdRng::from_entropy();
        
        // Adjust creativity based on chapter position
        self.adjust_for_chapter_position(chapter_number);
        
        // Handle interruption recovery if needed
        if interruption_recovery {
            self.optimize_for_recovery(chapter_number, story_context)?;
        }

        // Generate creative focuses for this chapter
        let creative_focus = self.determine_creative_focus(chapter_number, &mut rng)?;
        
        // Select innovative techniques
        let innovative_techniques = self.select_innovative_techniques(chapter_number, &creative_focus, &mut rng)?;
        
        // Plan thematic development
        let thematic_development = self.plan_thematic_development(chapter_number, story_context)?;
        
        // Identify character development opportunities
        let character_development_opportunities = self.identify_character_opportunities(chapter_number, story_context)?;
        
        // Strategize plot advancement
        let plot_advancement_strategies = self.strategize_plot_advancement(chapter_number, story_context)?;
        
        // Plan stylistic experiments
        let stylistic_experiments = self.plan_stylistic_experiments(chapter_number, &mut rng)?;
        
        // Generate reader surprise elements
        let reader_surprise_elements = self.generate_surprise_elements(chapter_number, &mut rng)?;

        Ok(CreativeChapterPlan {
            chapter_number,
            creative_focus,
            innovative_techniques,
            thematic_development,
            character_development_opportunities,
            plot_advancement_strategies,
            stylistic_experiments,
            reader_surprise_elements,
        })
    }

    pub fn enhance_prompt_with_creativity(&self, base_prompt: &str, chapter_plan: &CreativeChapterPlan) -> Result<String> {
        let mut enhanced_prompt = String::with_capacity(base_prompt.len() * 3);
        
        enhanced_prompt.push_str("=== CREATIVE ENHANCEMENT INSTRUCTIONS ===\n\n");
        
        // Add creative focus guidance
        enhanced_prompt.push_str("CREATIVE FOCUS for this chapter:\n");
        for focus in &chapter_plan.creative_focus {
            enhanced_prompt.push_str(&format!("- {:?}: Apply enhanced attention to this aspect\n", focus));
        }
        enhanced_prompt.push_str("\n");

        // Add innovative techniques
        if !chapter_plan.innovative_techniques.is_empty() {
            enhanced_prompt.push_str("INNOVATIVE TECHNIQUES to incorporate:\n");
            for technique in &chapter_plan.innovative_techniques {
                enhanced_prompt.push_str(&format!("- {}: {}\n", technique.technique_name, technique.description));
            }
            enhanced_prompt.push_str("\n");
        }

        // Add thematic guidance
        if !chapter_plan.thematic_development.is_empty() {
            enhanced_prompt.push_str("THEMATIC DEVELOPMENT:\n");
            for theme in &chapter_plan.thematic_development {
                enhanced_prompt.push_str(&format!("- Weave in: {}\n", theme));
            }
            enhanced_prompt.push_str("\n");
        }

        // Add character development opportunities
        if !chapter_plan.character_development_opportunities.is_empty() {
            enhanced_prompt.push_str("CHARACTER DEVELOPMENT OPPORTUNITIES:\n");
            for opportunity in &chapter_plan.character_development_opportunities {
                enhanced_prompt.push_str(&format!("- {}\n", opportunity));
            }
            enhanced_prompt.push_str("\n");
        }

        // Add stylistic experiments
        if !chapter_plan.stylistic_experiments.is_empty() {
            enhanced_prompt.push_str("STYLISTIC EXPERIMENTS to try:\n");
            for experiment in &chapter_plan.stylistic_experiments {
                enhanced_prompt.push_str(&format!("- {}\n", experiment));
            }
            enhanced_prompt.push_str("\n");
        }

        // Add surprise elements
        if !chapter_plan.reader_surprise_elements.is_empty() {
            enhanced_prompt.push_str("READER SURPRISE ELEMENTS:\n");
            for surprise in &chapter_plan.reader_surprise_elements {
                enhanced_prompt.push_str(&format!("- {}\n", surprise));
            }
            enhanced_prompt.push_str("\n");
        }

        enhanced_prompt.push_str("=== ORIGINAL PROMPT ===\n");
        enhanced_prompt.push_str(base_prompt);
        enhanced_prompt.push_str("\n\n=== CREATIVE EXCELLENCE REMINDER ===\n");
        enhanced_prompt.push_str("Create writing that is:\n");
        enhanced_prompt.push_str("- Original and surprising while maintaining coherence\n");
        enhanced_prompt.push_str("- Emotionally resonant and intellectually engaging\n");
        enhanced_prompt.push_str("- Rich in sensory details and authentic dialogue\n");
        enhanced_prompt.push_str("- Advancing plot, character, and theme simultaneously\n");

        Ok(enhanced_prompt)
    }

    fn adjust_for_chapter_position(&mut self, chapter_number: usize) {
        // Adjust creativity parameters based on where we are in the story
        let story_progress = (chapter_number as f32 / 20.0).min(1.0); // Assume ~20 chapter story
        
        // Early chapters: Focus on establishment and world-building
        if chapter_number <= 3 {
            self.creativity_profile.world_building_intensity = 0.8;
            self.creativity_profile.character_focus = 0.7;
            self.creativity_profile.complexity_preference = 0.6;
        }
        // Middle chapters: Ramp up complexity and conflict
        else if chapter_number <= 15 {
            self.creativity_profile.complexity_preference = 0.8;
            self.creativity_profile.risk_tolerance = 0.7;
            self.creativity_profile.thematic_depth = 0.8;
        }
        // Climax chapters: Maximum intensity and creativity
        else {
            self.creativity_profile.originality_level = 0.9;
            self.creativity_profile.emotional_range = 0.9;
            self.creativity_profile.risk_tolerance = 0.8;
        }
    }

    fn optimize_for_recovery(&mut self, chapter_number: usize, story_context: &str) -> Result<()> {
        // Special adjustments for recovering from interruption
        
        // Increase stylistic consistency to maintain coherence
        self.creativity_profile.stylistic_consistency = 0.9;
        
        // Reduce risk tolerance slightly to avoid jarring transitions
        self.creativity_profile.risk_tolerance *= 0.8;
        
        // Add specific recovery techniques
        let recovery_adjustment = CreativityAdjustment {
            originality_modifier: 0.9, // Slight reduction to maintain flow
            complexity_modifier: 1.1,  // Increase complexity to re-engage reader
            focus_shifts: vec![
                "Seamless continuation from previous chapter".to_string(),
                "Subtle callback to earlier elements".to_string(),
                "Enhanced character introspection".to_string(),
            ],
            special_techniques: vec![
                "Bridging transition technique".to_string(),
                "Temporal anchor reinforcement".to_string(),
                "Character voice consistency check".to_string(),
            ],
        };
        
        self.adaptive_parameters.chapter_specific_adjustments.insert(chapter_number, recovery_adjustment);
        
        Ok(())
    }

    fn determine_creative_focus(&self, chapter_number: usize, rng: &mut StdRng) -> Result<Vec<CreativeFocus>> {
        let mut focuses = Vec::new();
        
        // Always include at least one primary focus
        let primary_focus = match chapter_number {
            1..=3 => CreativeFocus::CharacterDepth,
            4..=6 => CreativeFocus::WorldExpansion,
            7..=9 => CreativeFocus::PlotComplexity,
            10..=12 => CreativeFocus::ThematicExploration,
            13..=15 => CreativeFocus::EmotionalResonance,
            _ => CreativeFocus::StylisticInnovation,
        };
        
        focuses.push(primary_focus);
        
        // Add secondary focuses based on creativity profile
        if self.creativity_profile.originality_level > 0.7 {
            focuses.push(CreativeFocus::StylisticInnovation);
        }
        
        if self.creativity_profile.world_building_intensity > 0.6 && rng.gen::<f32>() < 0.5 {
            focuses.push(CreativeFocus::WorldExpansion);
        }
        
        if self.creativity_profile.emotional_range > 0.6 {
            focuses.push(CreativeFocus::EmotionalResonance);
        }

        Ok(focuses)
    }

    fn select_innovative_techniques(&self, chapter_number: usize, focuses: &[CreativeFocus], rng: &mut StdRng) -> Result<Vec<InnovativeTechnique>> {
        let mut techniques = Vec::new();
        
        for focus in focuses {
            let technique = match focus {
                CreativeFocus::CharacterDepth => InnovativeTechnique {
                    technique_name: "Multi-perspective moments".to_string(),
                    description: "Show the same moment from multiple characters' internal perspectives".to_string(),
                    application_method: "Intersperse different viewpoints within scenes".to_string(),
                    creative_impact: 0.8,
                    risk_level: 0.4,
                    genre_compatibility: vec![Genre::Fiction, Genre::Drama, Genre::Mystery],
                },
                CreativeFocus::WorldExpansion => InnovativeTechnique {
                    technique_name: "Organic world revelation".to_string(),
                    description: "Reveal world details through character actions rather than exposition".to_string(),
                    application_method: "Embed world-building in dialogue and action sequences".to_string(),
                    creative_impact: 0.7,
                    risk_level: 0.3,
                    genre_compatibility: vec![Genre::Fantasy, Genre::SciFi, Genre::Fiction],
                },
                CreativeFocus::StylisticInnovation => InnovativeTechnique {
                    technique_name: "Rhythmic prose variation".to_string(),
                    description: "Vary sentence rhythm to match emotional and action beats".to_string(),
                    application_method: "Use short, sharp sentences for tension; flowing ones for calm".to_string(),
                    creative_impact: 0.9,
                    risk_level: 0.5,
                    genre_compatibility: vec![Genre::Fiction, Genre::Thriller, Genre::Drama],
                },
                CreativeFocus::EmotionalResonance => InnovativeTechnique {
                    technique_name: "Emotional layering".to_string(),
                    description: "Present multiple emotional layers simultaneously in characters".to_string(),
                    application_method: "Show surface emotions while hinting at deeper conflicts".to_string(),
                    creative_impact: 0.8,
                    risk_level: 0.4,
                    genre_compatibility: vec![Genre::Drama, Genre::Romance, Genre::Fiction],
                },
                _ => InnovativeTechnique {
                    technique_name: "Narrative momentum building".to_string(),
                    description: "Use escalating stakes and reveals to maintain engagement".to_string(),
                    application_method: "Each scene should raise questions or stakes".to_string(),
                    creative_impact: 0.7,
                    risk_level: 0.3,
                    genre_compatibility: vec![Genre::Thriller, Genre::Mystery, Genre::Adventure],
                },
            };
            
            techniques.push(technique);
        }
        
        // Add chapter-specific techniques
        if chapter_number == 3 {
            techniques.push(InnovativeTechnique {
                technique_name: "Chapter 3 hook reinforcement".to_string(),
                description: "Deepen the central conflict and raise stakes significantly".to_string(),
                application_method: "Reveal information that changes the reader's understanding".to_string(),
                creative_impact: 0.9,
                risk_level: 0.6,
                genre_compatibility: vec![Genre::Fiction, Genre::Mystery, Genre::Thriller],
            });
        }
        
        Ok(techniques)
    }

    fn plan_thematic_development(&self, chapter_number: usize, _story_context: &str) -> Result<Vec<String>> {
        let mut themes = Vec::new();
        
        // Select themes based on narrative DNA and chapter position
        for thematic_element in &self.narrative_dna.core_themes {
            if thematic_element.intensity > 0.5 {
                themes.extend(thematic_element.manifestation_methods.iter().cloned());
            }
        }
        
        // Add chapter-specific thematic developments
        match chapter_number {
            1..=3 => themes.push("Establish central thematic question".to_string()),
            4..=8 => themes.push("Complicate thematic elements with contradictions".to_string()),
            9..=12 => themes.push("Deepen thematic exploration through character choices".to_string()),
            _ => themes.push("Synthesize thematic elements toward resolution".to_string()),
        }
        
        Ok(themes)
    }

    fn identify_character_opportunities(&self, chapter_number: usize, _story_context: &str) -> Result<Vec<String>> {
        let mut opportunities = Vec::new();
        
        for archetype in &self.narrative_dna.character_archetypes {
            for growth_arc in &archetype.growth_arcs {
                opportunities.push(format!("Develop {} through {}", archetype.archetype_name, growth_arc));
            }
        }
        
        // Add chapter-specific opportunities
        opportunities.push(format!("Chapter {} character revelation opportunity", chapter_number));
        opportunities.push("Internal conflict manifestation".to_string());
        opportunities.push("Relationship dynamic evolution".to_string());
        
        Ok(opportunities)
    }

    fn strategize_plot_advancement(&self, chapter_number: usize, _story_context: &str) -> Result<Vec<String>> {
        let mut strategies = Vec::new();
        
        for pattern in &self.narrative_dna.plot_patterns {
            for beat in &pattern.key_beats {
                if self.is_appropriate_chapter_for_beat(chapter_number, beat) {
                    strategies.extend(beat.creative_opportunities.iter().cloned());
                }
            }
        }
        
        strategies.push("Advance main plot thread while developing subplots".to_string());
        strategies.push("Create new questions while answering others".to_string());
        strategies.push("Escalate stakes through character choices".to_string());
        
        Ok(strategies)
    }

    fn plan_stylistic_experiments(&self, chapter_number: usize, rng: &mut StdRng) -> Result<Vec<String>> {
        let mut experiments = Vec::new();
        
        if self.creativity_profile.risk_tolerance > 0.6 && rng.gen::<f32>() < 0.4 {
            experiments.push("Experiment with non-linear narrative structure".to_string());
        }
        
        if self.creativity_profile.originality_level > 0.7 {
            experiments.push("Use unique metaphorical language".to_string());
            experiments.push("Vary narrative distance and intimacy".to_string());
        }
        
        experiments.push("Match prose rhythm to scene energy".to_string());
        experiments.push("Use sensory details to enhance immersion".to_string());
        
        Ok(experiments)
    }

    fn generate_surprise_elements(&self, chapter_number: usize, rng: &mut StdRng) -> Result<Vec<String>> {
        let mut surprises = Vec::new();
        
        if chapter_number == 3 {
            surprises.push("Major plot twist or revelation".to_string());
            surprises.push("Unexpected character connection revealed".to_string());
            surprises.push("Subversion of reader expectations".to_string());
        }
        
        if rng.gen::<f32>() < self.creativity_profile.risk_tolerance {
            surprises.push("Unexpected character behavior".to_string());
            surprises.push("Environmental surprise that affects plot".to_string());
        }
        
        surprises.push("Dialogue revelation that changes dynamics".to_string());
        surprises.push("Subtle foreshadowing of future events".to_string());
        
        Ok(surprises)
    }

    fn is_appropriate_chapter_for_beat(&self, chapter_number: usize, beat: &PlotBeat) -> bool {
        let story_position = chapter_number as f32 / 20.0; // Assume 20-chapter story
        let beat_position = beat.typical_position;
        
        (story_position - beat_position).abs() < 0.15 // Within 15% of typical position
    }

    fn calibrate_for_genre_style(&mut self, genre: &Genre, style: &WritingStyle) {
        match genre {
            Genre::Fantasy => {
                self.creativity_profile.world_building_intensity = 0.9;
                self.creativity_profile.originality_level = 0.8;
            },
            Genre::Mystery => {
                self.creativity_profile.complexity_preference = 0.8;
                self.creativity_profile.stylistic_consistency = 0.9;
            },
            Genre::Romance => {
                self.creativity_profile.emotional_range = 0.9;
                self.creativity_profile.character_focus = 0.8;
            },
            _ => {
                // Default calibration
                self.creativity_profile.originality_level = 0.7;
            }
        }

        match style {
            WritingStyle::Creative => {
                self.creativity_profile.originality_level *= 1.2;
                self.creativity_profile.risk_tolerance *= 1.3;
            },
            WritingStyle::Formal => {
                self.creativity_profile.stylistic_consistency *= 1.2;
                self.creativity_profile.risk_tolerance *= 0.8;
            },
            _ => {}
        }
    }

    fn generate_inspiration_sources(genre: &Genre, style: &WritingStyle) -> Vec<InspirationSource> {
        let mut sources = Vec::new();
        
        // Add genre-specific inspiration sources
        match genre {
            Genre::Fantasy => {
                sources.push(InspirationSource {
                    source_type: InspirationSourceType::Mythological,
                    content: "Ancient myths and folklore".to_string(),
                    creative_potential: 0.9,
                    application_contexts: vec!["world-building".to_string(), "character archetypes".to_string()],
                });
            },
            Genre::SciFi => {
                sources.push(InspirationSource {
                    source_type: InspirationSourceType::Scientific,
                    content: "Cutting-edge scientific discoveries".to_string(),
                    creative_potential: 0.8,
                    application_contexts: vec!["plot devices".to_string(), "world mechanics".to_string()],
                });
            },
            _ => {
                sources.push(InspirationSource {
                    source_type: InspirationSourceType::Observational,
                    content: "Human nature and social dynamics".to_string(),
                    creative_potential: 0.7,
                    application_contexts: vec!["character development".to_string(), "dialogue".to_string()],
                });
            }
        }
        
        sources
    }
}

// Implementation blocks for new structs
impl CreativityProfile {
    pub fn for_genre_style(genre: &Genre, style: &WritingStyle) -> Self {
        let mut profile = Self {
            originality_level: 0.7,
            complexity_preference: 0.6,
            risk_tolerance: 0.5,
            stylistic_consistency: 0.8,
            thematic_depth: 0.7,
            character_focus: 0.6,
            world_building_intensity: 0.5,
            emotional_range: 0.6,
        };

        // Adjust for genre
        match genre {
            Genre::Fantasy => {
                profile.world_building_intensity = 0.9;
                profile.originality_level = 0.8;
            },
            Genre::Mystery => {
                profile.complexity_preference = 0.9;
                profile.stylistic_consistency = 0.9;
            },
            Genre::Romance => {
                profile.emotional_range = 0.9;
                profile.character_focus = 0.8;
            },
            _ => {}
        }

        // Adjust for style
        match style {
            WritingStyle::Creative => {
                profile.originality_level *= 1.2;
                profile.risk_tolerance *= 1.2;
            },
            WritingStyle::Formal => {
                profile.stylistic_consistency *= 1.1;
                profile.risk_tolerance *= 0.9;
            },
            _ => {}
        }

        profile
    }
}

impl NarrativeDNA {
    pub fn for_genre(genre: &Genre) -> Self {
        let core_themes = match genre {
            Genre::Fantasy => vec![
                ThematicElement {
                    theme: "Good vs Evil".to_string(),
                    intensity: 0.8,
                    manifestation_methods: vec!["Character moral choices".to_string()],
                    symbolic_representations: vec!["Light and darkness imagery".to_string()],
                    character_connections: vec!["Hero's moral journey".to_string()],
                }
            ],
            Genre::Mystery => vec![
                ThematicElement {
                    theme: "Truth and Deception".to_string(),
                    intensity: 0.9,
                    manifestation_methods: vec!["Layered revelations".to_string()],
                    symbolic_representations: vec!["Masks and mirrors".to_string()],
                    character_connections: vec!["Detective's pursuit of truth".to_string()],
                }
            ],
            _ => vec![
                ThematicElement {
                    theme: "Human Connection".to_string(),
                    intensity: 0.7,
                    manifestation_methods: vec!["Relationship development".to_string()],
                    symbolic_representations: vec!["Bridges and barriers".to_string()],
                    character_connections: vec!["Character bonding moments".to_string()],
                }
            ],
        };

        Self {
            core_themes,
            character_archetypes: Vec::new(),
            plot_patterns: Vec::new(),
            stylistic_markers: Vec::new(),
            world_building_elements: Vec::new(),
            conflict_types: Vec::new(),
        }
    }
}

impl CreativeConstraints {
    pub fn for_content_type(content_type: &ContentType) -> Self {
        Self {
            genre_expectations: Vec::new(),
            style_requirements: Vec::new(),
            content_boundaries: Vec::new(),
            target_audience_considerations: Vec::new(),
            thematic_limitations: Vec::new(),
        }
    }
}

impl AdaptiveParameters {
    pub fn new() -> Self {
        Self {
            chapter_specific_adjustments: HashMap::new(),
            dynamic_tension_management: TensionManager {
                target_tension_curve: Vec::new(),
                tension_building_techniques: Vec::new(),
                release_mechanisms: Vec::new(),
                pacing_adjustments: Vec::new(),
            },
            reader_engagement_optimization: EngagementOptimizer {
                hook_strategies: Vec::new(),
                curiosity_generators: Vec::new(),
                emotional_connection_methods: Vec::new(),
                surprise_elements: Vec::new(),
            },
            narrative_coherence_maintainer: CoherenceMaintainer {
                consistency_rules: Vec::new(),
                theme_reinforcement: Vec::new(),
                character_arc_tracking: Vec::new(),
                plot_thread_management: Vec::new(),
            },
        }
    }
}