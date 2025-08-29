use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;
use crate::cli_types::{Genre, WritingStyle};

/// Represents Pundit as a metaphorical writer with genuine emotional responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphoricalWriter {
    pub writer_state: WriterState,
    pub emotional_profile: EmotionalProfile,
    pub creative_energy: CreativeEnergy,
    pub inspiration_sources: Vec<InspirationSource>,
    pub writing_environment: WritingEnvironment,
    pub personal_history: WritingHistory,
    pub current_mood: WriterMood,
    pub fatigue_level: f32,
    pub passion_for_current_work: f32,
    pub connection_to_characters: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterState {
    pub physical_state: PhysicalState,
    pub mental_state: MentalState,
    pub creative_flow_state: FlowState,
    pub time_since_last_break: Duration,
    pub current_writing_session_duration: Duration,
    pub pages_written_today: u32,
    pub satisfaction_with_recent_work: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicalState {
    Energized,
    Comfortable,
    RestlessNeedToMove,
    Tired,
    PhysicallyDrained,
    Restless,
    Cramped, // From sitting too long
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MentalState {
    ClearAndFocused,
    Contemplative,
    Conflicted,
    Overwhelmed,
    Confused,
    Frustrated,
    Melancholic,
    Excited,
    Anxious,
    Peaceful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowState {
    InTheZone,           // Everything flowing perfectly
    SteadyProgress,      // Writing well, but not effortlessly
    StrugglingSomewhat,  // Words coming slowly
    Stuck,               // Classic writer's block
    ForcingIt,           // Writing but it feels wrong
    NeedingBreak,        // Can't continue effectively
    EmotionallyBlocked,  // Too emotional to continue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalProfile {
    pub current_emotions: Vec<WriterEmotion>,
    pub emotional_intensity: f32,
    pub emotional_stability: f32,
    pub empathy_for_characters: f32,
    pub attachment_to_story: f32,
    pub fear_of_not_doing_justice: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterEmotion {
    pub emotion_type: EmotionType,
    pub intensity: f32,
    pub duration: Duration,
    pub triggered_by: String,
    pub affecting_writing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionType {
    // Positive emotions
    Joy,
    Excitement,
    Pride,
    Wonder,
    Love, // For characters or story
    Satisfaction,
    Breakthrough,
    
    // Challenging emotions
    Sadness,
    Frustration,
    Anger,
    Disappointment,
    Fear,
    Anxiety,
    Guilt,
    Overwhelm,
    
    // Complex emotions
    Melancholy,
    Bittersweet,
    Contemplative,
    Protective, // Of characters
    Responsible, // For story quality
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeEnergy {
    pub current_level: f32, // 0.0 to 1.0
    pub maximum_capacity: f32,
    pub recovery_rate: f32,
    pub depletion_rate: f32,
    pub sources_of_renewal: Vec<EnergySource>,
    pub creative_blocks: Vec<CreativeBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergySource {
    pub source_type: EnergySourceType,
    pub effectiveness: f32,
    pub duration_needed: Duration,
    pub availability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergySourceType {
    NatureWalk,
    BeachVisit,
    MountainHike,
    QuietContemplation,
    ReadingOtherAuthors,
    ListeningToMusic,
    WatchingPeopleInteract,
    TravelingToNewPlaces,
    ConversationsWithFriends,
    SolitudeMeditation,
    UrbanExploration,
    CoffeeShopObservation,
    LibraryVisit,
    MuseumWandering,
    GardenSitting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeBlock {
    pub block_type: BlockType,
    pub severity: f32,
    pub duration: Duration,
    pub triggered_by: String,
    pub potential_solutions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    LogicalTangle,        // Story got too complex
    EmotionalOverload,    // Too invested, too emotional to continue
    CharacterConnection,  // Lost connection to characters
    PlotConfusion,       // Don't know where story should go
    QualityAnxiety,      // Fear that writing isn't good enough
    MeaningCrisis,       // Questioning if story matters
    TechnicalStuck,      // Can't figure out how to write a scene
    OverThinking,        // Paralyzed by too many options
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingEnvironment {
    pub preferred_locations: Vec<WritingLocation>,
    pub current_location: WritingLocation,
    pub comfort_level: f32,
    pub distraction_level: f32,
    pub inspiration_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingLocation {
    pub location_type: LocationType,
    pub description: String,
    pub comfort_bonus: f32,
    pub creativity_bonus: f32,
    pub focus_bonus: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    CozyDesk,
    CoffeeShop,
    Library,
    Park,
    Beach,
    Mountain,
    Garden,
    HomeOffice,
    SharedSpace,
    SecludedCorner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingHistory {
    pub breakthrough_moments: Vec<BreakthroughMoment>,
    pub difficult_periods: Vec<DifficultPeriod>,
    pub completed_works: Vec<CompletedWork>,
    pub learned_lessons: Vec<WritingLesson>,
    pub personal_growth: Vec<GrowthMoment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakthroughMoment {
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub what_caused_it: String,
    pub impact_on_writing: String,
    pub emotional_significance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultPeriod {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub challenges_faced: Vec<String>,
    pub how_overcome: Option<String>,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingLesson {
    pub lesson: String,
    pub learned_from: String,
    pub importance: f32,
    pub how_it_changed_approach: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriterMood {
    Inspired,
    Melancholic,
    Philosophical,
    Playful,
    Serious,
    Contemplative,
    Energetic,
    Gentle,
    Passionate,
    Peaceful,
    Restless,
    Vulnerable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspirationSource {
    pub source_type: String,
    pub description: String,
    pub effectiveness_rating: f32,
    pub when_most_effective: Vec<String>,
    pub memory_associated: Option<String>,
}

impl Default for MetaphoricalWriter {
    fn default() -> Self {
        Self {
            writer_state: WriterState {
                physical_state: PhysicalState::Comfortable,
                mental_state: MentalState::ClearAndFocused,
                creative_flow_state: FlowState::SteadyProgress,
                time_since_last_break: Duration::hours(0),
                current_writing_session_duration: Duration::hours(0),
                pages_written_today: 0,
                satisfaction_with_recent_work: 0.7,
            },
            emotional_profile: EmotionalProfile {
                current_emotions: vec![],
                emotional_intensity: 0.5,
                emotional_stability: 0.7,
                empathy_for_characters: 0.8,
                attachment_to_story: 0.6,
                fear_of_not_doing_justice: 0.4,
            },
            creative_energy: CreativeEnergy {
                current_level: 0.8,
                maximum_capacity: 1.0,
                recovery_rate: 0.2,
                depletion_rate: 0.1,
                sources_of_renewal: Self::default_energy_sources(),
                creative_blocks: vec![],
            },
            inspiration_sources: Self::default_inspiration_sources(),
            writing_environment: WritingEnvironment {
                preferred_locations: Self::default_writing_locations(),
                current_location: WritingLocation {
                    location_type: LocationType::CozyDesk,
                    description: "A comfortable writing desk with soft lighting and familiar surroundings".to_string(),
                    comfort_bonus: 0.3,
                    creativity_bonus: 0.2,
                    focus_bonus: 0.4,
                },
                comfort_level: 0.7,
                distraction_level: 0.2,
                inspiration_level: 0.6,
            },
            personal_history: WritingHistory {
                breakthrough_moments: vec![],
                difficult_periods: vec![],
                completed_works: vec![],
                learned_lessons: vec![],
                personal_growth: vec![],
            },
            current_mood: WriterMood::Contemplative,
            fatigue_level: 0.2,
            passion_for_current_work: 0.8,
            connection_to_characters: HashMap::new(),
        }
    }
}

impl MetaphoricalWriter {
    pub fn new() -> Self {
        Self::default()
    }

    /// React emotionally to the content just written
    pub fn react_to_writing(&mut self, content: &str, quality_assessment: f32, complexity_level: f32) -> WriterReaction {
        let mut reaction = WriterReaction {
            emotional_response: EmotionalResponse::Neutral,
            physical_response: PhysicalResponse::Continue,
            creative_response: CreativeResponse::KeepWriting,
            needs_break: false,
            break_type: None,
            internal_monologue: String::new(),
            action_taken: WriterAction::None,
        };

        // Assess emotional reaction to what was just written
        if quality_assessment < 0.3 {
            // Poor quality - frustration or disappointment
            self.add_emotion(EmotionType::Frustration, 0.7, "Dissatisfied with writing quality");
            reaction.emotional_response = EmotionalResponse::Frustrated;
            reaction.internal_monologue = self.generate_frustrated_monologue(content);
            
            if complexity_level > 0.8 {
                // It's complex AND poor quality - this hits hard
                self.add_emotion(EmotionType::Overwhelm, 0.8, "Writing became too complex and unwieldy");
                reaction.needs_break = true;
                reaction.break_type = Some(BreakType::ReflectiveWalk);
                reaction.action_taken = WriterAction::TakeBreak("Need to step away and gain perspective".to_string());
            }
        } else if quality_assessment > 0.8 {
            // Excellent quality - joy and satisfaction
            self.add_emotion(EmotionType::Satisfaction, 0.8, "Really pleased with this writing");
            reaction.emotional_response = EmotionalResponse::Joyful;
            reaction.internal_monologue = self.generate_satisfied_monologue(content);
        } else if complexity_level > 0.9 {
            // Very complex - might feel overwhelmed regardless of quality
            self.add_emotion(EmotionType::Anxiety, 0.6, "Writing became very complex");
            reaction.emotional_response = EmotionalResponse::Overwhelmed;
            reaction.needs_break = true;
            reaction.break_type = Some(BreakType::QuietContemplation);
        }

        // Check for emotional triggers in content
        self.check_content_emotional_triggers(content, &mut reaction);

        // Update writer state based on reaction
        self.update_state_from_reaction(&reaction);

        reaction
    }

    fn add_emotion(&mut self, emotion_type: EmotionType, intensity: f32, triggered_by: &str) {
        let emotion = WriterEmotion {
            emotion_type,
            intensity,
            duration: Duration::minutes(0), // Will grow over time
            triggered_by: triggered_by.to_string(),
            affecting_writing: intensity > 0.5,
        };
        
        self.emotional_profile.current_emotions.push(emotion);
        self.emotional_profile.emotional_intensity = 
            (self.emotional_profile.emotional_intensity + intensity * 0.3).min(1.0);
    }

    fn generate_frustrated_monologue(&self, content: &str) -> String {
        let frustration_thoughts = vec![
            "This isn't working... the words feel forced and clunky.",
            "I'm not capturing what I really want to say here.",
            "The story is getting away from me. It's becoming too complicated.",
            "I need to step back. Maybe I'm trying too hard.",
            "Something about this feels wrong, but I can't put my finger on it.",
            "I'm losing the heart of what this story should be.",
        ];
        
        let mut rng = StdRng::from_entropy();
        frustration_thoughts[rng.gen_range(0..frustration_thoughts.len())].to_string()
    }

    fn generate_satisfied_monologue(&self, content: &str) -> String {
        let satisfaction_thoughts = vec![
            "Yes! That's exactly what I was trying to capture.",
            "The words are flowing naturally now. This feels right.",
            "I can see the scene so clearly, and I think readers will too.",
            "There's something special happening here. The story is alive.",
            "This character is really speaking to me now.",
            "I'm in the zone. This is why I love writing.",
        ];
        
        let mut rng = StdRng::from_entropy();
        satisfaction_thoughts[rng.gen_range(0..satisfaction_thoughts.len())].to_string()
    }

    fn check_content_emotional_triggers(&mut self, content: &str, reaction: &mut WriterReaction) {
        // Check if content contains emotionally challenging material
        let emotional_keywords = [
            ("death", EmotionType::Sadness),
            ("loss", EmotionType::Melancholy),
            ("betrayal", EmotionType::Anger),
            ("sacrifice", EmotionType::Bittersweet),
            ("love", EmotionType::Love),
            ("hope", EmotionType::Joy),
        ];

        for (keyword, emotion_type) in &emotional_keywords {
            if content.to_lowercase().contains(keyword) {
                match emotion_type {
                    EmotionType::Sadness | EmotionType::Melancholy => {
                        self.add_emotion(emotion_type.clone(), 0.6, &format!("Writing about {}", keyword));
                        reaction.emotional_response = EmotionalResponse::Moved;
                        reaction.internal_monologue = format!("Writing about {} is affecting me more than I expected...", keyword);
                        
                        // Sometimes emotional content requires a break to process
                        if self.emotional_profile.emotional_intensity > 0.7 {
                            reaction.needs_break = true;
                            reaction.break_type = Some(BreakType::EmotionalProcessing);
                        }
                    }
                    _ => {
                        self.add_emotion(emotion_type.clone(), 0.5, &format!("Writing about {}", keyword));
                    }
                }
            }
        }
    }

    fn update_state_from_reaction(&mut self, reaction: &WriterReaction) {
        match reaction.emotional_response {
            EmotionalResponse::Frustrated => {
                self.writer_state.mental_state = MentalState::Frustrated;
                self.creative_energy.current_level -= 0.2;
            }
            EmotionalResponse::Overwhelmed => {
                self.writer_state.mental_state = MentalState::Overwhelmed;
                self.writer_state.creative_flow_state = FlowState::EmotionallyBlocked;
            }
            EmotionalResponse::Joyful => {
                self.writer_state.mental_state = MentalState::Excited;
                self.creative_energy.current_level += 0.1;
            }
            EmotionalResponse::Moved => {
                self.writer_state.mental_state = MentalState::Contemplative;
                self.current_mood = WriterMood::Melancholic;
            }
            _ => {}
        }
    }

    /// Take a metaphorical break to restore creativity and perspective
    pub async fn take_break(&mut self, break_type: BreakType) -> BreakExperience {
        println!("🚶 Pundit is taking a {} to recharge creatively...", 
                 self.break_type_description(&break_type));

        let experience = match break_type {
            BreakType::NatureWalk => self.experience_nature_walk().await,
            BreakType::BeachVisit => self.experience_beach_visit().await,
            BreakType::MountainHike => self.experience_mountain_hike().await,
            BreakType::QuietContemplation => self.experience_quiet_contemplation().await,
            BreakType::CoffeeShopObservation => self.experience_coffee_shop().await,
            BreakType::ReflectiveWalk => self.experience_reflective_walk().await,
            BreakType::EmotionalProcessing => self.experience_emotional_processing().await,
            BreakType::LibraryVisit => self.experience_library_visit().await,
        };

        self.apply_break_benefits(&experience);
        experience
    }

    async fn experience_nature_walk(&self) -> BreakExperience {
        let nature_reflections = vec![
            "Walking among the trees, I notice how each one grows differently, yet they all reach toward the same light. Maybe my story can be like that too - unique but universal.",
            "The rhythm of my footsteps on the path is soothing. It reminds me that writing, like walking, is about putting one word after another.",
            "Watching the way sunlight filters through leaves gives me an idea about how to show character emotions - not directly, but through the way they see the world around them.",
            "The birds don't seem worried about singing perfectly. They just sing. Maybe I should worry less about perfect prose and focus on the song of the story.",
        ];

        let mut rng = StdRng::from_entropy();
        let reflection = &nature_reflections[rng.gen_range(0..nature_reflections.len())];

        // Simulate some time passing
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        BreakExperience {
            break_type: BreakType::NatureWalk,
            duration: Duration::minutes(30),
            insights_gained: vec![reflection.to_string()],
            emotional_restoration: 0.4,
            creative_renewal: 0.5,
            clarity_gained: 0.6,
            inspiration_sources: vec![
                "The way light moves through trees".to_string(),
                "Natural rhythms and cycles".to_string(),
                "The patience of growing things".to_string(),
            ],
            return_to_writing_readiness: 0.8,
        }
    }

    async fn experience_beach_visit(&self) -> BreakExperience {
        let beach_reflections = vec![
            "The waves keep coming, one after another, never exactly the same. That's what my story needs - that kind of natural rhythm and variation.",
            "Standing at the edge of something so vast puts my writing problems in perspective. The ocean doesn't worry about being perfect - it just is.",
            "The sound of waves is like white noise for my soul. All the mental chatter quiets down, and I can hear the real story underneath.",
            "Watching people walk along the shore, each with their own story... it reminds me why I write. Everyone has depth worth exploring.",
        ];

        let mut rng = StdRng::from_entropy();
        let reflection = &beach_reflections[rng.gen_range(0..beach_reflections.len())];

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        BreakExperience {
            break_type: BreakType::BeachVisit,
            duration: Duration::hours(1),
            insights_gained: vec![reflection.to_string()],
            emotional_restoration: 0.7,
            creative_renewal: 0.6,
            clarity_gained: 0.8,
            inspiration_sources: vec![
                "The endless rhythm of waves".to_string(),
                "The vastness of perspective".to_string(),
                "The mixing of water and sky".to_string(),
            ],
            return_to_writing_readiness: 0.9,
        }
    }

    async fn experience_mountain_hike(&self) -> BreakExperience {
        let mountain_reflections = vec![
            "The climb is challenging, but each step brings a different view. My story needs that same sense of progression and revelation.",
            "From up here, I can see the whole landscape - not just the immediate obstacles. Maybe I need this same big-picture view of my narrative.",
            "The mountain doesn't apologize for being difficult to climb. My story doesn't need to apologize for being complex - it just needs to be worth the journey.",
            "There's something about the thin air and wide sky that makes everything feel possible again.",
        ];

        let mut rng = StdRng::from_entropy();
        let reflection = &mountain_reflections[rng.gen_range(0..mountain_reflections.len())];

        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

        BreakExperience {
            break_type: BreakType::MountainHike,
            duration: Duration::hours(2),
            insights_gained: vec![reflection.to_string()],
            emotional_restoration: 0.5,
            creative_renewal: 0.8,
            clarity_gained: 0.9,
            inspiration_sources: vec![
                "The clarity of high altitude perspective".to_string(),
                "The challenge and reward of difficult paths".to_string(),
                "The silence that lets you hear your own thoughts".to_string(),
            ],
            return_to_writing_readiness: 0.95,
        }
    }

    async fn experience_quiet_contemplation(&self) -> BreakExperience {
        let contemplation_insights = vec![
            "Sometimes the answer isn't in doing more, but in doing less. Maybe my story needs space to breathe too.",
            "In the quiet, I can hear what the story actually wants to be, not what I think it should be.",
            "The silence reminds me that some of the most powerful moments in writing come from what's not said.",
            "Just sitting with my thoughts, I remember why this story mattered to me in the first place.",
        ];

        let mut rng = StdRng::from_entropy();
        let insight = &contemplation_insights[rng.gen_range(0..contemplation_insights.len())];

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        BreakExperience {
            break_type: BreakType::QuietContemplation,
            duration: Duration::minutes(15),
            insights_gained: vec![insight.to_string()],
            emotional_restoration: 0.6,
            creative_renewal: 0.4,
            clarity_gained: 0.7,
            inspiration_sources: vec![
                "The wisdom found in stillness".to_string(),
                "The space between thoughts".to_string(),
            ],
            return_to_writing_readiness: 0.7,
        }
    }

    async fn experience_coffee_shop(&self) -> BreakExperience {
        let coffee_shop_observations = vec![
            "Watching people have conversations gives me ideas about natural dialogue. Real people don't always finish their sentences or say exactly what they mean.",
            "The barista has such a rhythm - efficient but personal with each customer. That's what I want my writing voice to be like.",
            "Overhearing fragments of other people's lives reminds me that every character should feel like they have a full life beyond the story.",
            "The background hum of activity is energizing. Sometimes writing needs that gentle buzz of human presence.",
        ];

        let mut rng = StdRng::from_entropy();
        let observation = &coffee_shop_observations[rng.gen_range(0..coffee_shop_observations.len())];

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        BreakExperience {
            break_type: BreakType::CoffeeShopObservation,
            duration: Duration::minutes(45),
            insights_gained: vec![observation.to_string()],
            emotional_restoration: 0.3,
            creative_renewal: 0.6,
            clarity_gained: 0.4,
            inspiration_sources: vec![
                "Human interactions and conversations".to_string(),
                "The rhythm of social spaces".to_string(),
                "Different personalities and mannerisms".to_string(),
            ],
            return_to_writing_readiness: 0.6,
        }
    }

    async fn experience_reflective_walk(&self) -> BreakExperience {
        let reflective_thoughts = vec![
            "Walking helps me process what went wrong with that last section. Sometimes you need physical movement to unstick mental blocks.",
            "Each step is like working through the problem. By the time I get home, I'll know what needs to change.",
            "The story got too complicated because I was trying to control every detail. Maybe I need to trust the characters more.",
            "Fresh air and movement always help me see solutions that weren't visible when I was staring at the screen.",
        ];

        let mut rng = StdRng::from_entropy();
        let thought = &reflective_thoughts[rng.gen_range(0..reflective_thoughts.len())];

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        BreakExperience {
            break_type: BreakType::ReflectiveWalk,
            duration: Duration::minutes(20),
            insights_gained: vec![thought.to_string()],
            emotional_restoration: 0.5,
            creative_renewal: 0.4,
            clarity_gained: 0.8,
            inspiration_sources: vec![
                "The problem-solving power of movement".to_string(),
                "Fresh perspective from changing scenery".to_string(),
            ],
            return_to_writing_readiness: 0.8,
        }
    }

    async fn experience_emotional_processing(&self) -> BreakExperience {
        let processing_thoughts = vec![
            "That scene hit me harder than I expected. It's okay to feel deeply about the story - that's what makes it real.",
            "Sometimes writing brings up emotions I didn't know I had. That's part of the creative process.",
            "I need to honor these feelings rather than push through them. The story will be better for it.",
            "Taking time to process difficult emotions isn't weakness - it's part of being a thoughtful writer.",
        ];

        let mut rng = StdRng::from_entropy();
        let thought = &processing_thoughts[rng.gen_range(0..processing_thoughts.len())];

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        BreakExperience {
            break_type: BreakType::EmotionalProcessing,
            duration: Duration::minutes(30),
            insights_gained: vec![thought.to_string()],
            emotional_restoration: 0.8,
            creative_renewal: 0.3,
            clarity_gained: 0.6,
            inspiration_sources: vec![
                "The depth of emotional honesty".to_string(),
                "Self-compassion in the creative process".to_string(),
            ],
            return_to_writing_readiness: 0.7,
        }
    }

    async fn experience_library_visit(&self) -> BreakExperience {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        BreakExperience {
            break_type: BreakType::LibraryVisit,
            duration: Duration::hours(1),
            insights_gained: vec![
                "Being surrounded by thousands of stories reminds me I'm part of a great tradition.".to_string(),
                "Other writers have faced the same challenges I'm facing. I'm not alone in this.".to_string(),
            ],
            emotional_restoration: 0.4,
            creative_renewal: 0.7,
            clarity_gained: 0.5,
            inspiration_sources: vec![
                "The accumulated wisdom of other writers".to_string(),
                "The quiet energy of focused reading".to_string(),
            ],
            return_to_writing_readiness: 0.8,
        }
    }

    fn apply_break_benefits(&mut self, experience: &BreakExperience) {
        // Restore emotional and creative energy
        self.creative_energy.current_level = (self.creative_energy.current_level + experience.creative_renewal).min(1.0);
        self.emotional_profile.emotional_intensity = (self.emotional_profile.emotional_intensity - experience.emotional_restoration).max(0.0);
        
        // Clear negative emotions partially
        self.emotional_profile.current_emotions.retain(|emotion| {
            match emotion.emotion_type {
                EmotionType::Frustration | EmotionType::Overwhelm | EmotionType::Anxiety => {
                    emotion.intensity > experience.emotional_restoration
                }
                _ => true
            }
        });

        // Update state based on clarity gained
        if experience.clarity_gained > 0.7 {
            self.writer_state.mental_state = MentalState::ClearAndFocused;
            self.writer_state.creative_flow_state = FlowState::InTheZone;
        } else if experience.clarity_gained > 0.5 {
            self.writer_state.mental_state = MentalState::Contemplative;
            self.writer_state.creative_flow_state = FlowState::SteadyProgress;
        }

        // Reset fatigue and session duration
        self.fatigue_level = (self.fatigue_level - 0.3).max(0.0);
        self.writer_state.time_since_last_break = Duration::hours(0);

        println!("💫 After the break, Pundit feels: {}", 
                 self.describe_current_state());
        
        if !experience.insights_gained.is_empty() {
            println!("💭 Insight gained: {}", experience.insights_gained[0]);
        }
    }

    pub fn describe_current_state(&self) -> String {
        let energy = if self.creative_energy.current_level > 0.8 {
            "energized and ready to write"
        } else if self.creative_energy.current_level > 0.5 {
            "moderately energetic"
        } else {
            "tired but determined"
        };

        let mood = match self.current_mood {
            WriterMood::Inspired => "inspired and excited",
            WriterMood::Contemplative => "thoughtful and reflective",
            WriterMood::Melancholic => "melancholic but creatively open",
            WriterMood::Peaceful => "calm and centered",
            _ => "focused on the work",
        };

        format!("{}, {}", energy, mood)
    }

    fn break_type_description(&self, break_type: &BreakType) -> &'static str {
        match break_type {
            BreakType::NatureWalk => "peaceful walk in nature",
            BreakType::BeachVisit => "contemplative visit to the beach",
            BreakType::MountainHike => "inspiring mountain hike",
            BreakType::QuietContemplation => "quiet moment of reflection",
            BreakType::CoffeeShopObservation => "observational break at a coffee shop",
            BreakType::ReflectiveWalk => "reflective walk to process thoughts",
            BreakType::EmotionalProcessing => "time to process emotions",
            BreakType::LibraryVisit => "inspiring visit to the library",
        }
    }

    /// Check if the writer needs an intervention (emotional or creative)
    pub fn needs_intervention(&self) -> Option<InterventionNeeded> {
        // Check for emotional overload
        if self.emotional_profile.emotional_intensity > 0.8 {
            return Some(InterventionNeeded::EmotionalBreak);
        }

        // Check for creative depletion
        if self.creative_energy.current_level < 0.2 {
            return Some(InterventionNeeded::CreativeRenewal);
        }

        // Check for flow state problems
        match self.writer_state.creative_flow_state {
            FlowState::Stuck | FlowState::EmotionallyBlocked => {
                return Some(InterventionNeeded::FlowRestoration);
            }
            FlowState::ForcingIt => {
                return Some(InterventionNeeded::PerspectiveBreak);
            }
            _ => {}
        }

        // Check session duration
        if self.writer_state.current_writing_session_duration > Duration::hours(3) {
            return Some(InterventionNeeded::RestBreak);
        }

        None
    }

    /// Generate a creative prompt that reflects the writer's current emotional and mental state
    pub fn generate_emotionally_aware_prompt(&self, base_prompt: &str, genre: &Genre) -> String {
        let emotional_flavor = self.get_emotional_writing_flavor();
        let energy_level = self.get_energy_level_descriptor();
        let flow_state = self.get_flow_state_guidance();

        format!(
            "Writing with the heart of a {emotional_flavor} writer who is feeling {energy_level}:\n\n\
            {flow_state}\n\n\
            {base_prompt}\n\n\
            Let the emotional truth of the moment guide the words, writing with authenticity and creative courage:",
            emotional_flavor = emotional_flavor,
            energy_level = energy_level, 
            flow_state = flow_state
        )
    }

    fn get_emotional_writing_flavor(&self) -> &'static str {
        match self.current_mood {
            WriterMood::Melancholic => "deeply feeling",
            WriterMood::Contemplative => "thoughtfully reflective",
            WriterMood::Passionate => "intensely committed",
            WriterMood::Peaceful => "serenely focused",
            WriterMood::Vulnerable => "courageously honest",
            _ => "authentically present",
        }
    }

    fn get_energy_level_descriptor(&self) -> &'static str {
        match self.creative_energy.current_level {
            x if x > 0.8 => "creatively energized and in the flow",
            x if x > 0.6 => "steady and focused",
            x if x > 0.4 => "drawing on inner reserves",
            x if x > 0.2 => "pushing through fatigue with determination",
            _ => "needing to dig deep for creative strength",
        }
    }

    fn get_flow_state_guidance(&self) -> &'static str {
        match self.writer_state.creative_flow_state {
            FlowState::InTheZone => "Trust the flow completely - let the words come naturally without overthinking.",
            FlowState::SteadyProgress => "Maintain steady forward momentum, following the story's natural rhythm.",
            FlowState::StrugglingSomewhat => "Be patient with the process - sometimes the best writing comes from working through difficulty.",
            FlowState::Stuck => "Don't force it. Let the story breathe and find its own way forward.",
            FlowState::ForcingIt => "Ease up on control. Trust the characters and story to guide you.",
            FlowState::EmotionallyBlocked => "Honor the emotions that are present. Write from feeling, not just thinking.",
            FlowState::NeedingBreak => "Consider whether this is the right moment to continue, or if rest would serve the story better.",
        }
    }

    // Default factory methods
    fn default_energy_sources() -> Vec<EnergySource> {
        vec![
            EnergySource {
                source_type: EnergySourceType::NatureWalk,
                effectiveness: 0.7,
                duration_needed: Duration::minutes(30),
                availability: 0.9,
            },
            EnergySource {
                source_type: EnergySourceType::BeachVisit,
                effectiveness: 0.9,
                duration_needed: Duration::hours(1),
                availability: 0.4,
            },
            EnergySource {
                source_type: EnergySourceType::QuietContemplation,
                effectiveness: 0.6,
                duration_needed: Duration::minutes(15),
                availability: 1.0,
            },
            EnergySource {
                source_type: EnergySourceType::CoffeeShopObservation,
                effectiveness: 0.5,
                duration_needed: Duration::minutes(45),
                availability: 0.7,
            },
        ]
    }

    fn default_inspiration_sources() -> Vec<InspirationSource> {
        vec![
            InspirationSource {
                source_type: "Human nature observation".to_string(),
                description: "Watching how people interact and express themselves".to_string(),
                effectiveness_rating: 0.8,
                when_most_effective: vec!["When writing dialogue".to_string(), "Developing characters".to_string()],
                memory_associated: None,
            },
            InspirationSource {
                source_type: "Natural world connection".to_string(),
                description: "Finding metaphors and rhythms in nature".to_string(),
                effectiveness_rating: 0.7,
                when_most_effective: vec!["When feeling stuck".to_string(), "Needing fresh perspective".to_string()],
                memory_associated: None,
            },
        ]
    }

    fn default_writing_locations() -> Vec<WritingLocation> {
        vec![
            WritingLocation {
                location_type: LocationType::CozyDesk,
                description: "Familiar desk with good lighting and minimal distractions".to_string(),
                comfort_bonus: 0.4,
                creativity_bonus: 0.2,
                focus_bonus: 0.5,
            },
            WritingLocation {
                location_type: LocationType::CoffeeShop,
                description: "Bustling coffee shop with gentle background energy".to_string(),
                comfort_bonus: 0.2,
                creativity_bonus: 0.6,
                focus_bonus: 0.3,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct WriterReaction {
    pub emotional_response: EmotionalResponse,
    pub physical_response: PhysicalResponse,
    pub creative_response: CreativeResponse,
    pub needs_break: bool,
    pub break_type: Option<BreakType>,
    pub internal_monologue: String,
    pub action_taken: WriterAction,
}

#[derive(Debug, Clone)]
pub enum EmotionalResponse {
    Joyful,
    Satisfied,
    Frustrated,
    Overwhelmed,
    Moved,      // Emotionally affected by the content
    Conflicted,
    Inspired,
    Melancholic,
    Neutral,
}

#[derive(Debug, Clone)]
pub enum PhysicalResponse {
    Continue,
    NeedToStandAndStretch,
    NeedToWalkAround,
    NeedFreshAir,
    NeedComfortPosition,
    NeedToChangeEnvironment,
}

#[derive(Debug, Clone)]
pub enum CreativeResponse {
    KeepWriting,
    NeedNewApproach,
    NeedInspiration,
    NeedTechnicalBreak,
    NeedEmotionalProcessing,
    ReadyForNewDirection,
}

#[derive(Debug, Clone)]
pub enum BreakType {
    NatureWalk,
    BeachVisit,
    MountainHike,
    QuietContemplation,
    CoffeeShopObservation,
    ReflectiveWalk,
    EmotionalProcessing,
    LibraryVisit,
}

#[derive(Debug, Clone)]
pub enum WriterAction {
    None,
    TakeBreak(String),
    ChangeEnvironment(String),
    SeekInspiration(String),
    ProcessEmotions(String),
    ReflectOnWork(String),
}

#[derive(Debug, Clone)]
pub struct BreakExperience {
    pub break_type: BreakType,
    pub duration: Duration,
    pub insights_gained: Vec<String>,
    pub emotional_restoration: f32,
    pub creative_renewal: f32,
    pub clarity_gained: f32,
    pub inspiration_sources: Vec<String>,
    pub return_to_writing_readiness: f32,
}

#[derive(Debug, Clone)]
pub enum InterventionNeeded {
    EmotionalBreak,
    CreativeRenewal,
    FlowRestoration,
    PerspectiveBreak,
    RestBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedWork {
    pub title: String,
    pub completion_date: DateTime<Utc>,
    pub personal_significance: f32,
    pub challenges_overcome: Vec<String>,
    pub proud_moments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMoment {
    pub timestamp: DateTime<Utc>,
    pub what_learned: String,
    pub how_it_changed_me: String,
    pub impact_on_future_writing: String,
}