use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::soul_memory::{SoulMemory, SoulMemoryConfig};
use crate::advanced_learning_system::AdvancedLearningSystem;
use crate::nonstop_learning_mode::{NonstopLearningMode, WorkGenerationResult};
use std::collections::VecDeque;

pub struct SoulMemoryManager {
    soul_memory: Option<Arc<SoulMemory>>,
    learning_system: Arc<Mutex<AdvancedLearningSystem>>,
    learning_buffer: Arc<Mutex<VecDeque<LearningEvent>>>,
    is_initialized: bool,
}

#[derive(Debug, Clone)]
pub struct LearningEvent {
    pub event_type: LearningEventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub content: String,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum LearningEventType {
    GenerationSuccess,
    GenerationFailure,
    UserFeedback,
    PatternDiscovery,
    SkillImprovement,
    CreativeBreakthrough,
}

impl SoulMemoryManager {
    pub fn new() -> Self {
        Self {
            soul_memory: None,
            learning_system: Arc::new(Mutex::new(AdvancedLearningSystem::new())),
            learning_buffer: Arc::new(Mutex::new(VecDeque::new())),
            is_initialized: false,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        if self.is_initialized {
            return Ok(());
        }

        // Initialize learning system from disk
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            if let Err(e) = learning_system.load_from_disk() {
                println!("⚠️ Could not load existing learning data: {}", e);
                println!("🆕 Starting with fresh learning state");
            }
        }

        // Initialize Soul Memory with default config
        let config = SoulMemoryConfig::default();
        let soul_memory = Arc::new(SoulMemory::new(config)?);

        // Try to restore from Soul Memory backup
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            if let Err(e) = soul_memory.integrate_with_learning_system(&mut learning_system).await {
                println!("⚠️ Could not restore from Soul Memory: {}", e);
            }
        }

        // Start background sync
        soul_memory.start_background_sync(self.learning_system.clone()).await?;

        self.soul_memory = Some(soul_memory);
        self.is_initialized = true;

        println!("🌟 Soul Memory Manager initialized successfully");
        Ok(())
    }

    pub fn get_learning_system(&self) -> Arc<Mutex<AdvancedLearningSystem>> {
        self.learning_system.clone()
    }

    pub async fn process_generation_result(&self, result: &WorkGenerationResult) -> Result<()> {
        if !self.is_initialized {
            return Ok(());
        }

        let event = LearningEvent {
            event_type: if result.success {
                LearningEventType::GenerationSuccess
            } else {
                LearningEventType::GenerationFailure
            },
            timestamp: chrono::Utc::now(),
            session_id: format!("work_{}", result.work_params.title.replace(" ", "_")),
            content: if let Some(ref error) = result.error_message {
                error.clone()
            } else {
                "Success".to_string()
            },
            metadata: {
                let mut meta = std::collections::HashMap::new();
                meta.insert("genre".to_string(), format!("{:?}", result.work_params.genre));
                meta.insert("style".to_string(), format!("{:?}", result.work_params.style));
                meta.insert("work_type".to_string(), format!("{:?}", result.work_params.work_type));
                meta.insert("generation_time".to_string(), result.generation_time_seconds.to_string());
                meta.insert("retry_count".to_string(), result.retry_count.to_string());
                meta
            },
        };

        // Add to learning buffer
        {
            let mut buffer = self.learning_buffer.lock().unwrap();
            buffer.push_back(event.clone());
            
            // Keep buffer manageable
            if buffer.len() > 1000 {
                buffer.pop_front();
            }
        }

        // Process immediately for important events
        if matches!(event.event_type, LearningEventType::GenerationFailure) {
            self.process_learning_event(&event).await?;
        } else {
            // Batch process success events
            self.maybe_process_batch().await?;
        }

        Ok(())
    }

    async fn maybe_process_batch(&self) -> Result<()> {
        let buffer_size = {
            let buffer = self.learning_buffer.lock().unwrap();
            buffer.len()
        };

        // Process batch if buffer is getting full or periodically
        if buffer_size >= 10 || self.should_process_batch().await {
            self.process_learning_batch().await?;
        }

        Ok(())
    }

    async fn should_process_batch(&self) -> bool {
        // Simple heuristic: process batch every 5 minutes or when buffer has items
        static mut LAST_BATCH_TIME: Option<std::time::Instant> = None;
        
        unsafe {
            let now = std::time::Instant::now();
            
            match LAST_BATCH_TIME {
                None => {
                    LAST_BATCH_TIME = Some(now);
                    true
                },
                Some(last_time) => {
                    if now.duration_since(last_time).as_secs() >= 300 {
                        LAST_BATCH_TIME = Some(now);
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    async fn process_learning_batch(&self) -> Result<()> {
        let events = {
            let mut buffer = self.learning_buffer.lock().unwrap();
            let events: Vec<LearningEvent> = buffer.drain(..).collect();
            events
        };

        for event in events {
            if let Err(e) = self.process_learning_event(&event).await {
                println!("⚠️ Failed to process learning event: {}", e);
                // Continue processing other events
            }
        }

        Ok(())
    }

    async fn process_learning_event(&self, event: &LearningEvent) -> Result<()> {
        let learning_system = self.learning_system.clone();
        
        match event.event_type {
            LearningEventType::GenerationSuccess => {
                // Update learning system with successful patterns
                self.learn_from_success(event).await?;
            },
            LearningEventType::GenerationFailure => {
                // Learn from failure
                self.learn_from_failure(event).await?;
            },
            LearningEventType::UserFeedback => {
                // Process user feedback
                self.process_user_feedback_event(event).await?;
            },
            LearningEventType::PatternDiscovery => {
                // Record new pattern discovery
                self.record_pattern_discovery(event).await?;
            },
            LearningEventType::SkillImprovement => {
                // Track skill improvement
                self.track_skill_improvement(event).await?;
            },
            LearningEventType::CreativeBreakthrough => {
                // Record creative breakthrough
                self.record_creative_breakthrough(event).await?;
            },
        }

        // Auto-save learning state periodically
        if self.should_auto_save().await {
            self.save_learning_state().await?;
        }

        Ok(())
    }

    async fn learn_from_success(&self, event: &LearningEvent) -> Result<()> {
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            
            // Extract successful techniques from the event
            let techniques = learning_system.extract_writing_techniques(&event.content);
            
            // Update long-term patterns
            learning_system.update_long_term_patterns(techniques);
            
            // Evolve stylistic capabilities
            learning_system.evolve_stylistic_capabilities(&event.content);
        }
        
        println!("✅ Learned from successful generation: {}", event.session_id);
        Ok(())
    }

    async fn learn_from_failure(&self, event: &LearningEvent) -> Result<()> {
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            
            // Process the error for learning
            learning_system.process_user_feedback(&event.content);
            
            // Detect any patterns in failures
            let patterns = learning_system.identify_new_patterns(&event.content);
            learning_system.update_long_term_patterns(patterns);
        }
        
        println!("🔍 Learned from generation failure: {}", event.session_id);
        Ok(())
    }

    async fn process_user_feedback_event(&self, event: &LearningEvent) -> Result<()> {
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            learning_system.process_user_feedback(&event.content);
        }
        
        println!("💬 Processed user feedback for session: {}", event.session_id);
        Ok(())
    }

    async fn record_pattern_discovery(&self, event: &LearningEvent) -> Result<()> {
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            let insights = learning_system.generate_session_insights();
            learning_system.update_recommendations(insights);
        }
        
        println!("🔍 Recorded pattern discovery: {}", event.content);
        Ok(())
    }

    async fn track_skill_improvement(&self, event: &LearningEvent) -> Result<()> {
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            learning_system.track_creative_evolution(&event.content);
        }
        
        println!("📈 Tracked skill improvement: {}", event.content);
        Ok(())
    }

    async fn record_creative_breakthrough(&self, event: &LearningEvent) -> Result<()> {
        {
            let mut learning_system = self.learning_system.lock().unwrap();
            let breakthroughs = learning_system.detect_creative_breakthroughs(&event.content);
            // Would process breakthrough data here
        }
        
        println!("🎨 Recorded creative breakthrough: {}", event.content);
        Ok(())
    }

    async fn should_auto_save(&self) -> bool {
        // Save every 50 processed events or every 10 minutes
        static mut EVENT_COUNT: u32 = 0;
        static mut LAST_SAVE_TIME: Option<std::time::Instant> = None;
        
        unsafe {
            EVENT_COUNT += 1;
            let now = std::time::Instant::now();
            
            if EVENT_COUNT >= 50 {
                EVENT_COUNT = 0;
                return true;
            }
            
            match LAST_SAVE_TIME {
                None => {
                    LAST_SAVE_TIME = Some(now);
                    true
                },
                Some(last_save) => {
                    if now.duration_since(last_save).as_secs() >= 600 {
                        LAST_SAVE_TIME = Some(now);
                        EVENT_COUNT = 0;
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    async fn save_learning_state(&self) -> Result<()> {
        // Save local learning state
        {
            let learning_system = self.learning_system.lock().unwrap();
            learning_system.save_to_disk()?;
        }

        // Trigger soul memory sync if available
        if let Some(ref soul_memory) = self.soul_memory {
            if let Err(e) = soul_memory.force_sync(&self.learning_system).await {
                println!("⚠️ Soul Memory sync failed: {}", e);
            }
        }

        Ok(())
    }

    pub async fn get_learning_insights(&self) -> Result<Vec<String>> {
        let learning_system = self.learning_system.lock().unwrap();
        Ok(learning_system.get_writing_quality_predictions(
            &crate::cli_types::Genre::Fiction,
            &crate::cli_types::WritingStyle::Creative
        ))
    }

    pub async fn force_soul_memory_sync(&self) -> Result<()> {
        if let Some(ref soul_memory) = self.soul_memory {
            soul_memory.force_sync(&self.learning_system).await?;
            println!("🔄 Soul Memory sync completed");
        } else {
            println!("⚠️ Soul Memory not initialized");
        }
        Ok(())
    }

    pub async fn get_soul_memory_status(&self) -> Option<String> {
        if let Some(ref soul_memory) = self.soul_memory {
            let status = soul_memory.get_sync_status().await;
            Some(format!(
                "Last sync: {:?}, Generation: {}, Failures: {}, Total syncs: {}",
                status.last_sync,
                status.current_generation,
                status.sync_failures,
                status.total_syncs
            ))
        } else {
            None
        }
    }

    // Integration with NonstopLearningMode
    pub fn integrate_with_nonstop_mode(&self, nonstop_mode: &mut NonstopLearningMode) {
        // Create non-mutex wrapped learning system for integration
        let learning_system_clone = {
            let system = self.learning_system.lock().unwrap();
            Arc::new(system.clone())
        };
        
        nonstop_mode.set_learning_systems(
            learning_system_clone,
            Arc::new(crate::master_intelligence_system::MasterIntelligenceSystem::new())
        );
    }
}