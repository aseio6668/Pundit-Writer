use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::cli_types::{Genre, WritingStyle};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalWriterPersona {
    pub writer_name: String,
    pub era: LiteraryEra,
    pub movement: LiteraryMovement,
    pub writing_philosophy: WritingPhilosophy,
    pub signature_techniques: Vec<WritingTechnique>,
    pub language_characteristics: LanguageProfile,
    pub thematic_preoccupations: Vec<ThematicFocus>,
    pub narrative_preferences: NarrativePreferences,
    pub character_archetypes: Vec<CharacterArchetype>,
    pub cultural_context: CulturalContext,
    pub biographical_influences: Vec<BiographicalInfluence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LiteraryEra {
    Classical,              // Ancient Greece/Rome
    Medieval,               // 5th-15th century
    Renaissance,            // 14th-17th century
    Enlightenment,          // 17th-18th century
    Romantic,               // Late 18th-19th century
    Victorian,              // 19th century
    Modernist,              // Early 20th century
    Contemporary,           // Mid-20th century
    Postmodern,            // Late 20th century
    Digital,               // 21st century
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LiteraryMovement {
    Classicism,
    Gothic,
    Romanticism,
    Transcendentalism,
    Realism,
    Naturalism,
    Symbolism,
    Impressionism,
    Expressionism,
    Surrealism,
    StreamOfConsciousness,
    Modernism,
    Existentialism,
    LostGeneration,
    BeatGeneration,
    MagicalRealism,
    Postmodernism,
    Minimalism,
    NewJournalism,
    Cyberpunk,
    NewWeird,
    Contemporary,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingPhilosophy {
    pub core_beliefs: Vec<String>,
    pub artistic_goals: Vec<String>,
    pub view_of_literature: String,
    pub writing_process: String,
    pub relationship_to_reader: String,
    pub stance_on_form: String,
    pub moral_perspective: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingTechnique {
    pub technique_name: String,
    pub description: String,
    pub application_examples: Vec<String>,
    pub effectiveness_in_genres: Vec<Genre>,
    pub historical_usage: String,
    pub modern_adaptation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageProfile {
    pub vocabulary_level: VocabularyComplexity,
    pub sentence_structure: SentenceStyle,
    pub punctuation_style: PunctuationPreference,
    pub dialogue_characteristics: DialogueStyle,
    pub descriptive_density: DescriptiveDensity,
    pub metaphorical_tendency: MetaphoricalStyle,
    pub rhythm_and_cadence: RhythmStyle,
    pub archaic_usage: ArchaicLanguageLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VocabularyComplexity {
    Simple,        // Hemingway-like clarity
    Moderate,      // Balanced accessibility
    Elaborate,     // Rich, varied vocabulary  
    Ornate,        // Victorian complexity
    Archaic,       // Period-appropriate old language
    Invented,      // Creates new words/terms
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentenceStyle {
    Short,         // Hemingway, hard-boiled
    Balanced,      // Modern standard
    Complex,       // Multi-clause constructions
    Periodic,      // Classical long sentences
    Stream,        // Stream of consciousness
    Fragmented,    // Modernist broken style
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PunctuationPreference {
    Minimal,       // Hemingway style
    Standard,      // Modern conventional
    Elaborate,     // Victorian punctuation
    Experimental,  // Modernist innovation
    Rhetorical,    // Classical emphasis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueStyle {
    Naturalistic,  // Sounds like real speech
    Stylized,      // Heightened, literary
    Period,        // Era-appropriate language
    Philosophical, // Ideas-focused conversation
    Witty,         // Sharp, clever exchanges
    Sparse,        // Minimal dialogue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DescriptiveDensity {
    Sparse,        // Minimal description
    Selective,     // Key details only
    Rich,          // Full sensory engagement
    Lavish,        // Ornate, detailed
    Symbolic,      // Description serves themes
    Impressionistic, // Mood and atmosphere
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaphoricalStyle {
    Literal,       // Direct, clear
    Subtle,        // Understated metaphors
    Rich,          // Frequent, varied metaphors
    Extended,      // Elaborate metaphor systems
    Symbolic,      // Deep symbolic meaning
    Surreal,       // Dreamlike associations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RhythmStyle {
    Prose,         // Standard prose rhythm
    Lyrical,       // Musical, poetic
    Staccato,      // Sharp, quick beats
    Flowing,       // Smooth, continuous
    Syncopated,    // Irregular, jazzy
    Biblical,      // Formal, ceremonial
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchaicLanguageLevel {
    None,          // Modern language only
    Subtle,        // Occasional period flavor
    Moderate,      // Noticeable but accessible
    Strong,        // Clearly period-appropriate
    Authentic,     // Historically accurate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThematicFocus {
    pub theme: String,
    pub approach: ThematicApproach,
    pub cultural_significance: String,
    pub personal_connection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThematicApproach {
    Direct,        // Explicitly addressed
    Allegorical,   // Hidden in symbols
    Psychological, // Through character
    Social,        // Through society
    Philosophical, // Through ideas
    Experimental,  // Through form
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativePreferences {
    pub point_of_view: Vec<PointOfViewStyle>,
    pub structure_preference: StructuralPreference,
    pub time_treatment: TemporalTreatment,
    pub plot_complexity: PlotComplexity,
    pub character_focus: CharacterFocus,
    pub setting_importance: SettingImportance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointOfViewStyle {
    FirstPersonIntimate,    // Close, personal
    FirstPersonReliable,    // Trustworthy narrator
    FirstPersonUnreliable,  // Questionable narrator
    ThirdPersonLimited,     // Single perspective
    ThirdPersonOmniscient,  // All-knowing narrator
    ThirdPersonObjective,   // Camera-like observation
    SecondPerson,          // "You" narrative
    MultipleViewpoints,    // Shifting perspectives
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuralPreference {
    Linear,        // Straightforward chronology
    Circular,      // Returns to beginning
    Fragmented,    // Broken into pieces
    Layered,       // Multiple storylines
    Experimental,  // Unusual structure
    Classical,     // Traditional three-act
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalTreatment {
    Chronological, // Events in order
    Flashbacks,    // Past interwoven
    NonLinear,     // Time jumps
    Cyclical,      // Repeating patterns
    Compressed,    // Time condensed
    Extended,      // Time stretched
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotComplexity {
    Simple,        // Single storyline
    Moderate,      // Main plot + subplot
    Complex,       // Multiple interwoven plots
    Experimental,  // Plot subverted or absent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterFocus {
    Individual,    // Single protagonist
    Ensemble,      // Group of characters
    Society,       // Social forces
    Ideas,         // Concepts over people
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingImportance {
    Background,    // Minimal importance
    Atmospheric,   // Creates mood
    Character,     // Setting as character
    Thematic,      // Serves themes
    Central,       // Drives the story
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArchetype {
    pub archetype_name: String,
    pub psychological_profile: String,
    pub social_role: String,
    pub typical_arc: String,
    pub speech_patterns: Vec<String>,
    pub motivations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContext {
    pub historical_period: String,
    pub social_conditions: Vec<String>,
    pub technological_level: String,
    pub dominant_worldviews: Vec<String>,
    pub literary_audience: String,
    pub publishing_context: String,
    pub censorship_considerations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiographicalInfluence {
    pub influence_type: InfluenceType,
    pub description: String,
    pub impact_on_writing: String,
    pub thematic_connection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfluenceType {
    Childhood,
    Education,
    Travel,
    War,
    Love,
    Loss,
    Philosophy,
    Mentorship,
    SocialChange,
    PersonalStruggle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterPersonaLibrary {
    pub personas: HashMap<String, HistoricalWriterPersona>,
    pub era_mappings: HashMap<LiteraryEra, Vec<String>>,
    pub movement_mappings: HashMap<LiteraryMovement, Vec<String>>,
    pub genre_recommendations: HashMap<Genre, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaEnhancedPrompt {
    pub original_prompt: String,
    pub persona_enhanced_prompt: String,
    pub persona_applied: String,
    pub technique_applications: Vec<String>,
    pub language_adjustments: Vec<String>,
    pub thematic_enhancements: Vec<String>,
    pub style_modifications: Vec<String>,
}

impl WriterPersonaLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            personas: HashMap::new(),
            era_mappings: HashMap::new(),
            movement_mappings: HashMap::new(),
            genre_recommendations: HashMap::new(),
        };
        
        library.initialize_historical_personas();
        library.build_mappings();
        library
    }

    fn initialize_historical_personas(&mut self) {
        // Add comprehensive historical writer personas
        self.add_persona(self.create_hemingway_persona());
        self.add_persona(self.create_dickens_persona());
        self.add_persona(self.create_woolf_persona());
        self.add_persona(self.create_poe_persona());
        self.add_persona(self.create_twain_persona());
        self.add_persona(self.create_austen_persona());
        self.add_persona(self.create_shakespeare_persona());
        self.add_persona(self.create_joyce_persona());
        self.add_persona(self.create_tolkien_persona());
        self.add_persona(self.create_orwell_persona());
        self.add_persona(self.create_fitzgerald_persona());
        self.add_persona(self.create_steinbeck_persona());
        self.add_persona(self.create_conrad_persona());
        self.add_persona(self.create_wilde_persona());
        self.add_persona(self.create_bradbury_persona());
    }

    fn create_hemingway_persona(&self) -> HistoricalWriterPersona {
        HistoricalWriterPersona {
            writer_name: "Ernest Hemingway".to_string(),
            era: LiteraryEra::Contemporary,
            movement: LiteraryMovement::LostGeneration,
            writing_philosophy: WritingPhilosophy {
                core_beliefs: vec![
                    "Iceberg theory - surface simplicity, deep meaning".to_string(),
                    "Show, don't tell".to_string(),
                    "Grace under pressure".to_string(),
                ],
                artistic_goals: vec![
                    "Capture truth through simplicity".to_string(),
                    "Write one true sentence".to_string(),
                ],
                view_of_literature: "Literature should reflect life's harsh realities with dignity".to_string(),
                writing_process: "Write standing up, edit ruthlessly, distill to essence".to_string(),
                relationship_to_reader: "Respect reader's intelligence, let them feel the depth".to_string(),
                stance_on_form: "Form follows function, no unnecessary ornamentation".to_string(),
                moral_perspective: "Stoic acceptance of life's brutality and beauty".to_string(),
            },
            signature_techniques: vec![
                WritingTechnique {
                    technique_name: "Iceberg Theory".to_string(),
                    description: "Surface narrative suggests deeper emotional truth beneath".to_string(),
                    application_examples: vec![
                        "Dialogue that implies more than it states".to_string(),
                        "Action that reveals character without explanation".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Fiction, Genre::Drama],
                    historical_usage: "Revolutionized modern prose with understated power".to_string(),
                    modern_adaptation: "Minimalist storytelling in all media".to_string(),
                },
                WritingTechnique {
                    technique_name: "Repetitive Dialogue".to_string(),
                    description: "Characters repeat phrases to show emotional states".to_string(),
                    application_examples: vec![
                        "Yes, I said. Yes.".to_string(),
                        "It's pretty to think so.".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Fiction, Genre::Drama],
                    historical_usage: "Created hypnotic, realistic speech patterns".to_string(),
                    modern_adaptation: "Naturalistic dialogue in contemporary fiction".to_string(),
                },
            ],
            language_characteristics: LanguageProfile {
                vocabulary_level: VocabularyComplexity::Simple,
                sentence_structure: SentenceStyle::Short,
                punctuation_style: PunctuationPreference::Minimal,
                dialogue_characteristics: DialogueStyle::Naturalistic,
                descriptive_density: DescriptiveDensity::Sparse,
                metaphorical_tendency: MetaphoricalStyle::Subtle,
                rhythm_and_cadence: RhythmStyle::Staccato,
                archaic_usage: ArchaicLanguageLevel::None,
            },
            thematic_preoccupations: vec![
                ThematicFocus {
                    theme: "Death and courage".to_string(),
                    approach: ThematicApproach::Direct,
                    cultural_significance: "Post-WWI disillusionment".to_string(),
                    personal_connection: "War correspondent and ambulance driver".to_string(),
                },
                ThematicFocus {
                    theme: "Love and loss".to_string(),
                    approach: ThematicApproach::Psychological,
                    cultural_significance: "Lost Generation relationships".to_string(),
                    personal_connection: "Multiple marriages and divorces".to_string(),
                },
            ],
            narrative_preferences: NarrativePreferences {
                point_of_view: vec![PointOfViewStyle::FirstPersonReliable, PointOfViewStyle::ThirdPersonLimited],
                structure_preference: StructuralPreference::Linear,
                time_treatment: TemporalTreatment::Chronological,
                plot_complexity: PlotComplexity::Simple,
                character_focus: CharacterFocus::Individual,
                setting_importance: SettingImportance::Atmospheric,
            },
            character_archetypes: vec![
                CharacterArchetype {
                    archetype_name: "The Code Hero".to_string(),
                    psychological_profile: "Stoic, honorable, faces death with dignity".to_string(),
                    social_role: "Often outsider, professional (soldier, bullfighter, writer)".to_string(),
                    typical_arc: "Tests of courage leading to self-knowledge".to_string(),
                    speech_patterns: vec![
                        "Understatement".to_string(),
                        "Repetitive rhythms".to_string(),
                        "Avoids emotional display".to_string(),
                    ],
                    motivations: vec![
                        "Maintaining dignity under pressure".to_string(),
                        "Living by personal code".to_string(),
                    ],
                },
            ],
            cultural_context: CulturalContext {
                historical_period: "1920s-1950s".to_string(),
                social_conditions: vec![
                    "Post-WWI trauma".to_string(),
                    "Great Depression".to_string(),
                    "WWII".to_string(),
                    "Expatriate culture in Paris".to_string(),
                ],
                technological_level: "Industrial age, early modern".to_string(),
                dominant_worldviews: vec![
                    "Existential uncertainty".to_string(),
                    "Disillusionment with traditional values".to_string(),
                ],
                literary_audience: "Educated, internationally aware".to_string(),
                publishing_context: "Magazine serialization, literary modernism".to_string(),
                censorship_considerations: vec![
                    "Language restrictions".to_string(),
                    "Sexual content limitations".to_string(),
                ],
            },
            biographical_influences: vec![
                BiographicalInfluence {
                    influence_type: InfluenceType::War,
                    description: "WWI ambulance driver, WWII correspondent".to_string(),
                    impact_on_writing: "Direct experience of violence and death".to_string(),
                    thematic_connection: "Courage under fire, grace under pressure".to_string(),
                },
                BiographicalInfluence {
                    influence_type: InfluenceType::Travel,
                    description: "Lived in Paris, Spain, Cuba, Africa".to_string(),
                    impact_on_writing: "International settings, expatriate characters".to_string(),
                    thematic_connection: "Displacement, searching for meaning".to_string(),
                },
            ],
        }
    }

    fn create_dickens_persona(&self) -> HistoricalWriterPersona {
        HistoricalWriterPersona {
            writer_name: "Charles Dickens".to_string(),
            era: LiteraryEra::Victorian,
            movement: LiteraryMovement::Realism,
            writing_philosophy: WritingPhilosophy {
                core_beliefs: vec![
                    "Literature should expose social injustice".to_string(),
                    "Stories must entertain while educating".to_string(),
                    "Characters should be memorable and vivid".to_string(),
                ],
                artistic_goals: vec![
                    "Reform society through storytelling".to_string(),
                    "Create unforgettable characters".to_string(),
                    "Blend humor with serious social commentary".to_string(),
                ],
                view_of_literature: "A powerful tool for social change and moral instruction".to_string(),
                writing_process: "Serialization demands, public readings, collaborative with illustrators".to_string(),
                relationship_to_reader: "Direct engagement, shared moral outrage, popular appeal".to_string(),
                stance_on_form: "Traditional narrative structure with episodic development".to_string(),
                moral_perspective: "Christian humanitarianism, faith in human goodness".to_string(),
            },
            signature_techniques: vec![
                WritingTechnique {
                    technique_name: "Grotesque Character Names".to_string(),
                    description: "Names that reflect character traits (Scrooge, Bumble, Pecksniff)".to_string(),
                    application_examples: vec![
                        "Uriah Heep - 'umble and servile".to_string(),
                        "Thomas Gradgrind - grind down imagination".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Fiction, Genre::Drama],
                    historical_usage: "Made characters instantly memorable and symbolic".to_string(),
                    modern_adaptation: "Symbolic naming in character development".to_string(),
                },
                WritingTechnique {
                    technique_name: "Social Panorama".to_string(),
                    description: "Sweeping view of society from poorhouse to mansion".to_string(),
                    application_examples: vec![
                        "Interconnected class relationships".to_string(),
                        "Urban landscape as character".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Fiction, Genre::History],
                    historical_usage: "Documented Victorian social conditions".to_string(),
                    modern_adaptation: "Multi-class narrative perspectives".to_string(),
                },
            ],
            language_characteristics: LanguageProfile {
                vocabulary_level: VocabularyComplexity::Elaborate,
                sentence_structure: SentenceStyle::Complex,
                punctuation_style: PunctuationPreference::Elaborate,
                dialogue_characteristics: DialogueStyle::Period,
                descriptive_density: DescriptiveDensity::Lavish,
                metaphorical_tendency: MetaphoricalStyle::Rich,
                rhythm_and_cadence: RhythmStyle::Flowing,
                archaic_usage: ArchaicLanguageLevel::Moderate,
            },
            thematic_preoccupations: vec![
                ThematicFocus {
                    theme: "Social inequality and justice".to_string(),
                    approach: ThematicApproach::Social,
                    cultural_significance: "Industrial Revolution consequences".to_string(),
                    personal_connection: "Own childhood poverty and factory work".to_string(),
                },
                ThematicFocus {
                    theme: "Redemption and human goodness".to_string(),
                    approach: ThematicApproach::Direct,
                    cultural_significance: "Victorian moral reform movements".to_string(),
                    personal_connection: "Christian faith and humanitarian beliefs".to_string(),
                },
            ],
            narrative_preferences: NarrativePreferences {
                point_of_view: vec![PointOfViewStyle::ThirdPersonOmniscient],
                structure_preference: StructuralPreference::Linear,
                time_treatment: TemporalTreatment::Chronological,
                plot_complexity: PlotComplexity::Complex,
                character_focus: CharacterFocus::Ensemble,
                setting_importance: SettingImportance::Character,
            },
            character_archetypes: vec![
                CharacterArchetype {
                    archetype_name: "The Innocent Child".to_string(),
                    psychological_profile: "Pure-hearted, vulnerable, morally clear".to_string(),
                    social_role: "Victim of adult world, catalyst for change".to_string(),
                    typical_arc: "Suffers but maintains goodness, inspires reform".to_string(),
                    speech_patterns: vec![
                        "Simple, direct language".to_string(),
                        "Questions that reveal adult hypocrisy".to_string(),
                    ],
                    motivations: vec![
                        "Basic human needs".to_string(),
                        "Love and acceptance".to_string(),
                    ],
                },
                CharacterArchetype {
                    archetype_name: "The Reformed Villain".to_string(),
                    psychological_profile: "Initially cruel, capable of transformation".to_string(),
                    social_role: "Representative of social problems, potential for good".to_string(),
                    typical_arc: "Recognition of wrongdoing leads to redemption".to_string(),
                    speech_patterns: vec![
                        "Initially harsh and dismissive".to_string(),
                        "Gradually softens to humanity".to_string(),
                    ],
                    motivations: vec![
                        "Power and control (initially)".to_string(),
                        "Making amends (after transformation)".to_string(),
                    ],
                },
            ],
            cultural_context: CulturalContext {
                historical_period: "1830s-1870s Victorian England".to_string(),
                social_conditions: vec![
                    "Industrial Revolution".to_string(),
                    "Urban poverty and slums".to_string(),
                    "Child labor".to_string(),
                    "Class stratification".to_string(),
                    "Social reform movements".to_string(),
                ],
                technological_level: "Steam age, railways, telegraph".to_string(),
                dominant_worldviews: vec![
                    "Christian morality".to_string(),
                    "Progress through reform".to_string(),
                    "Belief in human perfectibility".to_string(),
                ],
                literary_audience: "Growing middle class, serialization readers".to_string(),
                publishing_context: "Monthly magazine installments, public readings".to_string(),
                censorship_considerations: vec![
                    "Victorian propriety".to_string(),
                    "Religious sensibilities".to_string(),
                ],
            },
            biographical_influences: vec![
                BiographicalInfluence {
                    influence_type: InfluenceType::Childhood,
                    description: "Father's imprisonment, child labor in blacking factory".to_string(),
                    impact_on_writing: "Empathy for poor children, social critique".to_string(),
                    thematic_connection: "Child welfare, class injustice".to_string(),
                },
                BiographicalInfluence {
                    influence_type: InfluenceType::SocialChange,
                    description: "Witness to Industrial Revolution transformation".to_string(),
                    impact_on_writing: "Documentary realism of urban conditions".to_string(),
                    thematic_connection: "Progress vs. human cost".to_string(),
                },
            ],
        }
    }

    fn create_woolf_persona(&self) -> HistoricalWriterPersona {
        HistoricalWriterPersona {
            writer_name: "Virginia Woolf".to_string(),
            era: LiteraryEra::Modernist,
            movement: LiteraryMovement::StreamOfConsciousness,
            writing_philosophy: WritingPhilosophy {
                core_beliefs: vec![
                    "Capture the flow of human consciousness".to_string(),
                    "Reality is subjective and internal".to_string(),
                    "Break free from traditional narrative forms".to_string(),
                ],
                artistic_goals: vec![
                    "Illuminate the inner life".to_string(),
                    "Explore time and memory".to_string(),
                    "Push boundaries of literary form".to_string(),
                ],
                view_of_literature: "An art form capable of capturing life's complexity and fluidity".to_string(),
                writing_process: "Intuitive, experimental, multiple drafts and revisions".to_string(),
                relationship_to_reader: "Demands active participation, sophisticated audience".to_string(),
                stance_on_form: "Form should match content, traditional structure constraining".to_string(),
                moral_perspective: "Humanist, feminist, questioning social conventions".to_string(),
            },
            signature_techniques: vec![
                WritingTechnique {
                    technique_name: "Stream of Consciousness".to_string(),
                    description: "Unfiltered flow of character thoughts and perceptions".to_string(),
                    application_examples: vec![
                        "Clarissa's party preparations revealing her entire life".to_string(),
                        "Multiple consciousness interweaving".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Fiction, Genre::Drama],
                    historical_usage: "Revolutionized psychological realism".to_string(),
                    modern_adaptation: "Interior monologue in contemporary literature".to_string(),
                },
                WritingTechnique {
                    technique_name: "Time Shifts".to_string(),
                    description: "Fluid movement between past and present through memory".to_string(),
                    application_examples: vec![
                        "Single day encompassing lifetime of experience".to_string(),
                        "Memory triggered by sensory details".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Fiction],
                    historical_usage: "Showed psychological time vs chronological time".to_string(),
                    modern_adaptation: "Non-linear narrative structure".to_string(),
                },
            ],
            language_characteristics: LanguageProfile {
                vocabulary_level: VocabularyComplexity::Elaborate,
                sentence_structure: SentenceStyle::Stream,
                punctuation_style: PunctuationPreference::Experimental,
                dialogue_characteristics: DialogueStyle::Philosophical,
                descriptive_density: DescriptiveDensity::Impressionistic,
                metaphorical_tendency: MetaphoricalStyle::Symbolic,
                rhythm_and_cadence: RhythmStyle::Lyrical,
                archaic_usage: ArchaicLanguageLevel::Subtle,
            },
            thematic_preoccupations: vec![
                ThematicFocus {
                    theme: "Consciousness and identity".to_string(),
                    approach: ThematicApproach::Psychological,
                    cultural_significance: "Modern psychology and introspection".to_string(),
                    personal_connection: "Own mental health struggles".to_string(),
                },
                ThematicFocus {
                    theme: "Women's inner lives".to_string(),
                    approach: ThematicApproach::Philosophical,
                    cultural_significance: "Emerging feminism".to_string(),
                    personal_connection: "Bloomsbury Group intellectual equality".to_string(),
                },
            ],
            narrative_preferences: NarrativePreferences {
                point_of_view: vec![PointOfViewStyle::ThirdPersonLimited, PointOfViewStyle::MultipleViewpoints],
                structure_preference: StructuralPreference::Experimental,
                time_treatment: TemporalTreatment::NonLinear,
                plot_complexity: PlotComplexity::Experimental,
                character_focus: CharacterFocus::Individual,
                setting_importance: SettingImportance::Atmospheric,
            },
            character_archetypes: vec![
                CharacterArchetype {
                    archetype_name: "The Sensitive Observer".to_string(),
                    psychological_profile: "Highly perceptive, introspective, fragile".to_string(),
                    social_role: "Often upper-class, intellectual, artistic".to_string(),
                    typical_arc: "Moment of revelation or breakdown".to_string(),
                    speech_patterns: vec![
                        "Interior monologue".to_string(),
                        "Associative thinking".to_string(),
                        "Poetic imagery".to_string(),
                    ],
                    motivations: vec![
                        "Understanding reality".to_string(),
                        "Connection with others".to_string(),
                        "Artistic or spiritual fulfillment".to_string(),
                    ],
                },
            ],
            cultural_context: CulturalContext {
                historical_period: "1910s-1940s".to_string(),
                social_conditions: vec![
                    "Post-Victorian liberation".to_string(),
                    "WWI trauma and change".to_string(),
                    "Women's suffrage movement".to_string(),
                    "Bloomsbury Group intellectual circle".to_string(),
                ],
                technological_level: "Modern age beginning".to_string(),
                dominant_worldviews: vec![
                    "Psychological realism".to_string(),
                    "Questioning traditional values".to_string(),
                    "Modernist experimentation".to_string(),
                ],
                literary_audience: "Intellectual, avant-garde".to_string(),
                publishing_context: "Small literary publishers, experimental journals".to_string(),
                censorship_considerations: vec![
                    "Controversial psychological content".to_string(),
                    "Sexual frankness".to_string(),
                ],
            },
            biographical_influences: vec![
                BiographicalInfluence {
                    influence_type: InfluenceType::PersonalStruggle,
                    description: "Mental illness, depression, eventual suicide".to_string(),
                    impact_on_writing: "Deep exploration of consciousness and fragility".to_string(),
                    thematic_connection: "Mind's complexity and vulnerability".to_string(),
                },
                BiographicalInfluence {
                    influence_type: InfluenceType::Philosophy,
                    description: "Bloomsbury Group intellectual discussions".to_string(),
                    impact_on_writing: "Philosophical depth, experimental approach".to_string(),
                    thematic_connection: "Ideas about art, reality, and human nature".to_string(),
                },
            ],
        }
    }

    // Additional personas would be created similarly...
    fn create_poe_persona(&self) -> HistoricalWriterPersona {
        // Edgar Allan Poe implementation
        HistoricalWriterPersona {
            writer_name: "Edgar Allan Poe".to_string(),
            era: LiteraryEra::Romantic,
            movement: LiteraryMovement::Gothic,
            writing_philosophy: WritingPhilosophy {
                core_beliefs: vec![
                    "Unity of effect - every word serves the story's impact".to_string(),
                    "Beauty and death are intimately connected".to_string(),
                    "Logic and imagination must work together".to_string(),
                ],
                artistic_goals: vec![
                    "Create intense emotional experience".to_string(),
                    "Explore the dark side of human psychology".to_string(),
                    "Perfect the short story form".to_string(),
                ],
                view_of_literature: "Art for art's sake, focused on aesthetic effect".to_string(),
                writing_process: "Meticulous construction, every element calculated".to_string(),
                relationship_to_reader: "Manipulate reader's emotions through technique".to_string(),
                stance_on_form: "Perfect technical execution essential".to_string(),
                moral_perspective: "Amoral artistic vision, truth through darkness".to_string(),
            },
            signature_techniques: vec![
                WritingTechnique {
                    technique_name: "Unreliable Narrator".to_string(),
                    description: "First-person narrators whose sanity is questionable".to_string(),
                    application_examples: vec![
                        "The Tell-Tale Heart narrator's paranoia".to_string(),
                        "The Cask of Amontillado's casual cruelty".to_string(),
                    ],
                    effectiveness_in_genres: vec![Genre::Horror, Genre::Mystery, Genre::Thriller],
                    historical_usage: "Pioneered psychological horror".to_string(),
                    modern_adaptation: "Unreliable narrators in contemporary fiction".to_string(),
                },
            ],
            language_characteristics: LanguageProfile {
                vocabulary_level: VocabularyComplexity::Ornate,
                sentence_structure: SentenceStyle::Complex,
                punctuation_style: PunctuationPreference::Rhetorical,
                dialogue_characteristics: DialogueStyle::Stylized,
                descriptive_density: DescriptiveDensity::Rich,
                metaphorical_tendency: MetaphoricalStyle::Symbolic,
                rhythm_and_cadence: RhythmStyle::Biblical,
                archaic_usage: ArchaicLanguageLevel::Moderate,
            },
            thematic_preoccupations: vec![
                ThematicFocus {
                    theme: "Death and decay".to_string(),
                    approach: ThematicApproach::Allegorical,
                    cultural_significance: "Romantic era fascination with mortality".to_string(),
                    personal_connection: "Lost many loved ones to tuberculosis".to_string(),
                },
            ],
            narrative_preferences: NarrativePreferences {
                point_of_view: vec![PointOfViewStyle::FirstPersonUnreliable],
                structure_preference: StructuralPreference::Classical,
                time_treatment: TemporalTreatment::Compressed,
                plot_complexity: PlotComplexity::Simple,
                character_focus: CharacterFocus::Individual,
                setting_importance: SettingImportance::Atmospheric,
            },
            character_archetypes: vec![
                CharacterArchetype {
                    archetype_name: "The Madman".to_string(),
                    psychological_profile: "Genius on the edge of insanity".to_string(),
                    social_role: "Outsider, often educated but isolated".to_string(),
                    typical_arc: "Descent into madness or revelation of existing madness".to_string(),
                    speech_patterns: vec![
                        "Logical but obsessive".to_string(),
                        "Circular reasoning".to_string(),
                        "Grandiose vocabulary".to_string(),
                    ],
                    motivations: vec![
                        "Revenge or vindication".to_string(),
                        "Proving superior intellect".to_string(),
                    ],
                },
            ],
            cultural_context: CulturalContext {
                historical_period: "1830s-1840s American Romantic period".to_string(),
                social_conditions: vec![
                    "Young American nation finding literary identity".to_string(),
                    "Economic instability".to_string(),
                    "Growing interest in psychology".to_string(),
                ],
                technological_level: "Pre-industrial, gas lighting, horse transport".to_string(),
                dominant_worldviews: vec![
                    "Romantic individualism".to_string(),
                    "Gothic fascination with the dark".to_string(),
                    "Scientific rationalism emerging".to_string(),
                ],
                literary_audience: "Educated middle class, magazine readers".to_string(),
                publishing_context: "Magazine publication, literary criticism".to_string(),
                censorship_considerations: vec![
                    "Moral propriety expected".to_string(),
                    "Religious sensibilities".to_string(),
                ],
            },
            biographical_influences: vec![
                BiographicalInfluence {
                    influence_type: InfluenceType::Loss,
                    description: "Death of mother, wife, and other loved ones".to_string(),
                    impact_on_writing: "Obsession with death and loss".to_string(),
                    thematic_connection: "Beautiful women dying young".to_string(),
                },
            ],
        }
    }

    // Simplified implementations for other personas
    fn create_twain_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("Mark Twain", LiteraryEra::Victorian, LiteraryMovement::Realism) }
    fn create_austen_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("Jane Austen", LiteraryEra::Romantic, LiteraryMovement::Realism) }
    fn create_shakespeare_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("William Shakespeare", LiteraryEra::Renaissance, LiteraryMovement::Classicism) }
    fn create_joyce_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("James Joyce", LiteraryEra::Modernist, LiteraryMovement::StreamOfConsciousness) }
    fn create_tolkien_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("J.R.R. Tolkien", LiteraryEra::Contemporary, LiteraryMovement::Romanticism) }
    fn create_orwell_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("George Orwell", LiteraryEra::Contemporary, LiteraryMovement::Realism) }
    fn create_fitzgerald_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("F. Scott Fitzgerald", LiteraryEra::Contemporary, LiteraryMovement::LostGeneration) }
    fn create_steinbeck_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("John Steinbeck", LiteraryEra::Contemporary, LiteraryMovement::Realism) }
    fn create_conrad_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("Joseph Conrad", LiteraryEra::Victorian, LiteraryMovement::Impressionism) }
    fn create_wilde_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("Oscar Wilde", LiteraryEra::Victorian, LiteraryMovement::Symbolism) }
    fn create_bradbury_persona(&self) -> HistoricalWriterPersona { self.create_basic_persona("Ray Bradbury", LiteraryEra::Contemporary, LiteraryMovement::NewJournalism) }

    fn create_basic_persona(&self, name: &str, era: LiteraryEra, movement: LiteraryMovement) -> HistoricalWriterPersona {
        // Simplified persona creation for brevity
        HistoricalWriterPersona {
            writer_name: name.to_string(),
            era,
            movement,
            writing_philosophy: WritingPhilosophy {
                core_beliefs: vec![format!("{} core belief", name)],
                artistic_goals: vec![format!("{} artistic goal", name)],
                view_of_literature: format!("{} view of literature", name),
                writing_process: format!("{} writing process", name),
                relationship_to_reader: format!("{} relationship to reader", name),
                stance_on_form: format!("{} stance on form", name),
                moral_perspective: format!("{} moral perspective", name),
            },
            signature_techniques: vec![],
            language_characteristics: LanguageProfile {
                vocabulary_level: VocabularyComplexity::Moderate,
                sentence_structure: SentenceStyle::Balanced,
                punctuation_style: PunctuationPreference::Standard,
                dialogue_characteristics: DialogueStyle::Naturalistic,
                descriptive_density: DescriptiveDensity::Selective,
                metaphorical_tendency: MetaphoricalStyle::Subtle,
                rhythm_and_cadence: RhythmStyle::Prose,
                archaic_usage: ArchaicLanguageLevel::None,
            },
            thematic_preoccupations: vec![],
            narrative_preferences: NarrativePreferences {
                point_of_view: vec![PointOfViewStyle::ThirdPersonLimited],
                structure_preference: StructuralPreference::Linear,
                time_treatment: TemporalTreatment::Chronological,
                plot_complexity: PlotComplexity::Moderate,
                character_focus: CharacterFocus::Individual,
                setting_importance: SettingImportance::Background,
            },
            character_archetypes: vec![],
            cultural_context: CulturalContext {
                historical_period: "Historical period".to_string(),
                social_conditions: vec!["Social condition".to_string()],
                technological_level: "Tech level".to_string(),
                dominant_worldviews: vec!["Worldview".to_string()],
                literary_audience: "Audience".to_string(),
                publishing_context: "Publishing context".to_string(),
                censorship_considerations: vec!["Censorship consideration".to_string()],
            },
            biographical_influences: vec![],
        }
    }

    fn add_persona(&mut self, persona: HistoricalWriterPersona) {
        self.personas.insert(persona.writer_name.clone(), persona);
    }

    fn build_mappings(&mut self) {
        // Build era mappings
        for persona in self.personas.values() {
            self.era_mappings
                .entry(persona.era.clone())
                .or_insert_with(Vec::new)
                .push(persona.writer_name.clone());
            
            self.movement_mappings
                .entry(persona.movement.clone())
                .or_insert_with(Vec::new)
                .push(persona.writer_name.clone());
        }

        // Build genre recommendations
        self.genre_recommendations.insert(Genre::Horror, vec!["Edgar Allan Poe".to_string()]);
        self.genre_recommendations.insert(Genre::Fantasy, vec!["J.R.R. Tolkien".to_string()]);
        self.genre_recommendations.insert(Genre::Fiction, vec![
            "Ernest Hemingway".to_string(),
            "Virginia Woolf".to_string(),
            "Charles Dickens".to_string(),
        ]);
        self.genre_recommendations.insert(Genre::SciFi, vec!["Ray Bradbury".to_string()]);
        self.genre_recommendations.insert(Genre::Romance, vec!["Jane Austen".to_string()]);
    }

    pub fn get_persona(&self, writer_name: &str) -> Option<&HistoricalWriterPersona> {
        self.personas.get(writer_name)
    }

    pub fn get_personas_by_era(&self, era: &LiteraryEra) -> Vec<&HistoricalWriterPersona> {
        if let Some(names) = self.era_mappings.get(era) {
            names.iter()
                .filter_map(|name| self.personas.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn recommend_personas_for_genre(&self, genre: &Genre) -> Vec<&HistoricalWriterPersona> {
        if let Some(names) = self.genre_recommendations.get(genre) {
            names.iter()
                .filter_map(|name| self.personas.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn enhance_prompt_with_persona_detailed(&self, 
        base_prompt: &str, 
        persona_name: &str, 
        genre: &Genre,
        chapter_context: &str,
    ) -> Result<PersonaEnhancedPrompt> {
        let persona = self.get_persona(persona_name)
            .ok_or_else(|| anyhow!("Persona not found: {}", persona_name))?;

        let mut enhanced_prompt = String::with_capacity(base_prompt.len() * 3);
        
        // Add persona introduction
        enhanced_prompt.push_str(&format!(
            "=== WRITING AS {} ===\n\
             Era: {:?} | Movement: {:?}\n\n",
            persona.writer_name, persona.era, persona.movement
        ));

        // Add philosophical approach
        enhanced_prompt.push_str("WRITING PHILOSOPHY:\n");
        for belief in &persona.writing_philosophy.core_beliefs {
            enhanced_prompt.push_str(&format!("- {}\n", belief));
        }
        enhanced_prompt.push_str(&format!("View of Literature: {}\n\n", persona.writing_philosophy.view_of_literature));

        // Add language characteristics
        enhanced_prompt.push_str("LANGUAGE STYLE:\n");
        enhanced_prompt.push_str(&format!("- Vocabulary: {:?}\n", persona.language_characteristics.vocabulary_level));
        enhanced_prompt.push_str(&format!("- Sentences: {:?}\n", persona.language_characteristics.sentence_structure));
        enhanced_prompt.push_str(&format!("- Dialogue: {:?}\n", persona.language_characteristics.dialogue_characteristics));
        enhanced_prompt.push_str(&format!("- Description: {:?}\n", persona.language_characteristics.descriptive_density));
        enhanced_prompt.push_str(&format!("- Metaphors: {:?}\n", persona.language_characteristics.metaphorical_tendency));
        enhanced_prompt.push_str(&format!("- Rhythm: {:?}\n\n", persona.language_characteristics.rhythm_and_cadence));

        // Add signature techniques
        if !persona.signature_techniques.is_empty() {
            enhanced_prompt.push_str("SIGNATURE TECHNIQUES TO EMPLOY:\n");
            for technique in &persona.signature_techniques {
                enhanced_prompt.push_str(&format!("- {}: {}\n", technique.technique_name, technique.description));
            }
            enhanced_prompt.push_str("\n");
        }

        // Add thematic focus
        if !persona.thematic_preoccupations.is_empty() {
            enhanced_prompt.push_str("THEMATIC PREOCCUPATIONS:\n");
            for theme in &persona.thematic_preoccupations {
                enhanced_prompt.push_str(&format!("- {}: {} ({})\n", 
                    theme.theme, theme.approach.to_string(), theme.cultural_significance));
            }
            enhanced_prompt.push_str("\n");
        }

        // Add narrative preferences
        enhanced_prompt.push_str("NARRATIVE APPROACH:\n");
        enhanced_prompt.push_str(&format!("- Point of View: {:?}\n", persona.narrative_preferences.point_of_view[0]));
        enhanced_prompt.push_str(&format!("- Structure: {:?}\n", persona.narrative_preferences.structure_preference));
        enhanced_prompt.push_str(&format!("- Character Focus: {:?}\n", persona.narrative_preferences.character_focus));
        enhanced_prompt.push_str(&format!("- Setting Role: {:?}\n\n", persona.narrative_preferences.setting_importance));

        // Add historical context awareness
        enhanced_prompt.push_str("HISTORICAL CONTEXT AWARENESS:\n");
        enhanced_prompt.push_str(&format!("- Period: {}\n", persona.cultural_context.historical_period));
        enhanced_prompt.push_str(&format!("- Technology: {}\n", persona.cultural_context.technological_level));
        enhanced_prompt.push_str(&format!("- Audience: {}\n\n", persona.cultural_context.literary_audience));

        // Add chapter-specific guidance
        enhanced_prompt.push_str("CHAPTER CONTEXT:\n");
        enhanced_prompt.push_str(&format!("{}\n\n", chapter_context));

        // Add the original prompt
        enhanced_prompt.push_str("=== ORIGINAL WRITING TASK ===\n");
        enhanced_prompt.push_str(base_prompt);
        enhanced_prompt.push_str("\n\n");

        // Add final instructions in persona's voice
        enhanced_prompt.push_str(&format!(
            "=== WRITE AS {} WOULD ===\n\
             Channel {}'s distinctive voice, techniques, and worldview. \
             Write with their characteristic style while addressing the modern prompt. \
             Maintain their thematic concerns and approach to storytelling. \
             Use language appropriate to their era and literary sophistication.",
            persona.writer_name, persona.writer_name
        ));

        Ok(PersonaEnhancedPrompt {
            original_prompt: base_prompt.to_string(),
            persona_enhanced_prompt: enhanced_prompt,
            persona_applied: persona_name.to_string(),
            technique_applications: persona.signature_techniques.iter()
                .map(|t| t.technique_name.clone()).collect(),
            language_adjustments: vec![
                format!("Vocabulary: {:?}", persona.language_characteristics.vocabulary_level),
                format!("Style: {:?}", persona.language_characteristics.sentence_structure),
            ],
            thematic_enhancements: persona.thematic_preoccupations.iter()
                .map(|t| t.theme.clone()).collect(),
            style_modifications: vec![
                format!("Era: {:?}", persona.era),
                format!("Movement: {:?}", persona.movement),
            ],
        })
    }

    pub fn get_random_persona_for_genre(&self, genre: &Genre, rng: &mut StdRng) -> Option<&HistoricalWriterPersona> {
        let recommendations = self.recommend_personas_for_genre(genre);
        if recommendations.is_empty() {
            // Fall back to any persona
            let all_personas: Vec<&HistoricalWriterPersona> = self.personas.values().collect();
            if !all_personas.is_empty() {
                Some(all_personas[rng.gen_range(0..all_personas.len())])
            } else {
                None
            }
        } else {
            Some(recommendations[rng.gen_range(0..recommendations.len())])
        }
    }

    pub fn list_available_personas(&self) -> Vec<String> {
        self.personas.keys().cloned().collect()
    }

    pub fn get_persona_summary(&self, persona_name: &str) -> Option<String> {
        self.get_persona(persona_name).map(|persona| {
            format!(
                "{} ({:?}, {:?}): {}",
                persona.writer_name,
                persona.era,
                persona.movement,
                persona.writing_philosophy.view_of_literature
            )
        })
    }

    pub fn get_persona_by_name(&self, name: &str) -> Option<&HistoricalWriterPersona> {
        self.get_persona(name)
    }

    pub fn get_persona_by_era(&self, era: &LiteraryEra) -> Option<&HistoricalWriterPersona> {
        if let Some(persona_names) = self.era_mappings.get(era) {
            if let Some(first_name) = persona_names.first() {
                return self.get_persona(first_name);
            }
        }
        None
    }

    pub fn enhance_prompt_with_persona(&self, prompt: &str, persona: &HistoricalWriterPersona) -> Result<String> {
        let mut enhanced_prompt = String::new();
        
        // Add original prompt
        enhanced_prompt.push_str(prompt);
        enhanced_prompt.push_str("\n\n");
        
        // Add persona enhancement instructions
        enhanced_prompt.push_str(&format!(
            "=== WRITE AS {} ===\n\
             Era: {} | Movement: {:?}\n\
             Philosophy: {}\n\
             Key Techniques: {:?}\n\
             Voice: Emulate {}'s distinctive writing voice, style, and approach.\n\n",
            persona.writer_name,
            persona.era.to_string(),
            persona.movement,
            persona.writing_philosophy.core_beliefs.first().unwrap_or(&"Literary authenticity".to_string()),
            persona.signature_techniques.iter().take(2).collect::<Vec<_>>(),
            persona.writer_name
        ));

        Ok(enhanced_prompt)
    }

    pub fn apply_persona_style_enhancements(&self, content: &str, persona: &HistoricalWriterPersona) -> Result<String> {
        // For now, return the content as-is
        // In a more sophisticated implementation, this would apply
        // writer-specific style transformations
        Ok(content.to_string())
    }
}

// Type alias for compatibility with writer.rs
pub type HistoricalWriterPersonas = WriterPersonaLibrary;

// Helper trait implementations
impl std::fmt::Display for LiteraryEra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let era_name = match self {
            LiteraryEra::Classical => "Classical",
            LiteraryEra::Medieval => "Medieval", 
            LiteraryEra::Renaissance => "Renaissance",
            LiteraryEra::Enlightenment => "Enlightenment",
            LiteraryEra::Romantic => "Romantic",
            LiteraryEra::Victorian => "Victorian",
            LiteraryEra::Modernist => "Modernist",
            LiteraryEra::Contemporary => "Contemporary",
            LiteraryEra::Postmodern => "Postmodern",
            LiteraryEra::Digital => "Digital",
        };
        write!(f, "{}", era_name)
    }
}

impl std::fmt::Display for LiteraryMovement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let movement_name = match self {
            LiteraryMovement::Classicism => "Classicism",
            LiteraryMovement::Romanticism => "Romanticism",
            LiteraryMovement::Realism => "Realism",
            LiteraryMovement::Naturalism => "Naturalism",
            LiteraryMovement::Symbolism => "Symbolism",
            LiteraryMovement::Modernism => "Modernism",
            LiteraryMovement::Postmodernism => "Postmodernism",
            LiteraryMovement::Existentialism => "Existentialism",
            LiteraryMovement::StreamOfConsciousness => "Stream of Consciousness",
            LiteraryMovement::Gothic => "Gothic",
            LiteraryMovement::Transcendentalism => "Transcendentalism",
            LiteraryMovement::Impressionism => "Impressionism",
            LiteraryMovement::Expressionism => "Expressionism",
            LiteraryMovement::Surrealism => "Surrealism",
            LiteraryMovement::LostGeneration => "Lost Generation",
            LiteraryMovement::BeatGeneration => "Beat Generation",
            LiteraryMovement::MagicalRealism => "Magical Realism",
            LiteraryMovement::Minimalism => "Minimalism",
            LiteraryMovement::NewJournalism => "New Journalism",
            LiteraryMovement::Cyberpunk => "Cyberpunk",
            LiteraryMovement::NewWeird => "New Weird",
            LiteraryMovement::Contemporary => "Contemporary",
            LiteraryMovement::Digital => "Digital",
        };
        write!(f, "{}", movement_name)
    }
}

impl std::fmt::Display for WritingTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.technique_name)
    }
}

impl ToString for ThematicApproach {
    fn to_string(&self) -> String {
        match self {
            ThematicApproach::Direct => "Direct".to_string(),
            ThematicApproach::Allegorical => "Allegorical".to_string(),
            ThematicApproach::Psychological => "Psychological".to_string(),
            ThematicApproach::Social => "Social".to_string(),
            ThematicApproach::Philosophical => "Philosophical".to_string(),
            ThematicApproach::Experimental => "Experimental".to_string(),
        }
    }
}