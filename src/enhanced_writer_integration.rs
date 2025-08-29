use crate::advanced_learning_system::{AdvancedLearningSystem, WritingContext, LearningInsights, QualityPrediction, CreativityEnhancement};
use crate::cognitive_writing_engine::CognitiveWritingEngine;
use crate::neural_creativity_enhancer::NeuralCreativityEnhancer;
use crate::master_intelligence_system::MasterIntelligenceSystem;
use crate::enhanced_writer_system::EnhancedWriterSystem;
use crate::cli_types::{Genre, WritingStyle, BookSize};
use crate::adaptive_learning_engine::UserFeedback;
use crate::content::{Content, Section, SectionType, Book};
use crate::writer::AIClient;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use chrono::Utc;

/// Integration layer that connects all sophisticated AI writing systems to the core writer
pub struct EnhancedWriterIntegration {
    pub advanced_learning: Arc<Mutex<AdvancedLearningSystem>>,
    pub cognitive_engine: Arc<Mutex<CognitiveWritingEngine>>,
    pub creativity_enhancer: Arc<Mutex<NeuralCreativityEnhancer>>,
    pub master_intelligence: Arc<Mutex<MasterIntelligenceSystem>>,
    pub enhanced_writer: Arc<Mutex<EnhancedWriterSystem>>,
    pub session_id: String,
}

impl EnhancedWriterIntegration {
    pub fn new() -> Result<Self> {
        let session_id = format!("session_{}", Utc::now().timestamp());
        
        // Initialize all systems
        let mut advanced_learning = AdvancedLearningSystem::new();
        
        // Load existing learning data if available
        if let Err(e) = advanced_learning.load_from_disk() {
            eprintln!("⚠️  Could not load learning data: {}. Starting fresh.", e);
        }

        Ok(Self {
            advanced_learning: Arc::new(Mutex::new(advanced_learning)),
            cognitive_engine: Arc::new(Mutex::new(CognitiveWritingEngine::new())),
            creativity_enhancer: Arc::new(Mutex::new(NeuralCreativityEnhancer::new())),
            master_intelligence: Arc::new(Mutex::new(MasterIntelligenceSystem::new())),
            enhanced_writer: Arc::new(Mutex::new(EnhancedWriterSystem::new())),
            session_id,
        })
    }

    /// Enhanced content generation that uses all available AI systems
    pub async fn generate_enhanced_content(
        &self,
        client: &AIClient,
        model: &str,
        book: &Book,
        section_number: usize,
        section_type: &SectionType,
        base_prompt: &str,
        target_words: usize,
        context: &str,
    ) -> Result<String> {
        println!("🧠 Engaging advanced AI writing systems for section {}", section_number);

        // Create writing context
        let writing_context = WritingContext {
            genre: Genre::Fiction, // Default to Fiction
            style: WritingStyle::Creative, // Default to Creative
            target_audience: "General".to_string(),
            constraints: vec!["Maintain consistency".to_string(), "Follow outline".to_string()],
            purpose: "Chapter content generation".to_string(),
        };

        // 1. Use Advanced Learning System to enhance the prompt
        let enhanced_prompt = {
            let learning_system = self.advanced_learning.lock().unwrap();
            learning_system.generate_enhanced_prompt(
                base_prompt,
                &writing_context.genre,
                &writing_context.style,
                &writing_context,
            )
        };

        // 2. Get quality prediction before generation
        let quality_prediction = {
            let learning_system = self.advanced_learning.lock().unwrap();
            learning_system.predict_writing_quality(&enhanced_prompt, &writing_context.genre, &writing_context.style)
        };

        println!("📊 Quality prediction: {:.1}% confidence, {:.1}% predicted score", 
            quality_prediction.confidence_level * 100.0,
            quality_prediction.predicted_score * 100.0
        );

        // 3. Apply creativity enhancement if needed
        let creativity_enhanced_prompt = if quality_prediction.creativity_potential < 0.7 {
            let enhancement = {
                let learning_system = self.advanced_learning.lock().unwrap();
                learning_system.adaptive_creativity_boost(&enhanced_prompt, 0.8)
            };
            
            if enhancement.expected_impact > 0.1 {
                println!("🎨 Applying creativity enhancement (expected impact: {:.1}%)", 
                    enhancement.expected_impact * 100.0);
                format!("{}\n\nCREATIVITY ENHANCEMENT:\n{}", enhanced_prompt, 
                    enhancement.strategies.join(", "))
            } else {
                enhanced_prompt
            }
        } else {
            enhanced_prompt
        };

        // 4. Use Enhanced Writer System for final generation
        let generated_content = {
            let enhanced_writer = self.enhanced_writer.lock().unwrap();
            enhanced_writer.enhanced_generation_with_learning(
                client,
                model,
                &creativity_enhanced_prompt,
                section_number,
                &BookSize::Medium,
                &writing_context.genre,
                &writing_context.style,
                context,
            ).await?
        };

        // 5. Analyze generated content quality
        let quality_score = self.calculate_content_quality(&generated_content, &writing_context);

        // 6. Process learning from this generation
        let learning_insights = {
            let mut learning_system = self.advanced_learning.lock().unwrap();
            learning_system.process_writing_session(
                &self.session_id,
                writing_context.genre,
                writing_context.style,
                &generated_content,
                None, // No user feedback yet
                quality_score,
            )
        };

        // 7. Display learning insights
        if !learning_insights.session_insights.is_empty() {
            println!("💡 Learning insights: {}", learning_insights.session_insights[0]);
        }

        // 8. Auto-save learning progress
        if let Err(e) = self.auto_save_learning().await {
            eprintln!("⚠️  Could not auto-save learning progress: {}", e);
        }

        Ok(generated_content)
    }

    /// Process user feedback on generated content to improve future generations
    pub async fn process_user_feedback(
        &self,
        content: &str,
        feedback: UserFeedback,
        section_info: (usize, Genre, WritingStyle),
    ) -> Result<()> {
        let (section_number, genre, style) = section_info;

        // Calculate quality score based on feedback
        let quality_score = feedback.rating as f32 / 10.0; // Convert 1-10 rating to 0-1 score

        // Process through advanced learning system
        {
            let mut learning_system = self.advanced_learning.lock().unwrap();
            learning_system.process_writing_session(
                &self.session_id,
                genre,
                style,
                content,
                Some(feedback),
                quality_score,
            );
        }

        // Save updated learning
        self.auto_save_learning().await?;

        println!("✅ Feedback processed and learning updated");
        Ok(())
    }

    /// Get writing quality predictions for planning
    pub fn get_quality_predictions(&self, genre: &Genre, style: &WritingStyle) -> Vec<String> {
        let learning_system = self.advanced_learning.lock().unwrap();
        learning_system.get_writing_quality_predictions(genre, style)
    }

    /// Calculate content quality using multiple metrics
    fn calculate_content_quality(&self, content: &str, context: &WritingContext) -> f32 {
        // Basic quality metrics
        let word_count = content.split_whitespace().count();
        let sentence_count = content.matches(&['.', '!', '?'][..]).count();
        
        // Length appropriateness (penalty for too short/long content)
        let length_score = if word_count < 200 {
            word_count as f32 / 200.0
        } else if word_count > 2000 {
            1.0 - ((word_count - 2000) as f32 / 2000.0).min(0.5)
        } else {
            1.0
        };

        // Sentence variety (good if sentences vary in length)
        let variety_score = if sentence_count > 0 {
            let avg_words_per_sentence = word_count as f32 / sentence_count as f32;
            (avg_words_per_sentence / 20.0).min(1.0)
        } else {
            0.0
        };

        // Genre appropriateness (basic keyword analysis)
        let genre_score = match context.genre {
            Genre::Fantasy => {
                let fantasy_keywords = ["magic", "dragon", "wizard", "kingdom", "quest", "spell"];
                let keyword_count = fantasy_keywords.iter()
                    .map(|&kw| content.to_lowercase().matches(kw).count())
                    .sum::<usize>();
                (keyword_count as f32 / word_count as f32 * 100.0).min(1.0)
            },
            Genre::Mystery => {
                let mystery_keywords = ["clue", "investigate", "suspect", "mystery", "detective", "evidence"];
                let keyword_count = mystery_keywords.iter()
                    .map(|&kw| content.to_lowercase().matches(kw).count())
                    .sum::<usize>();
                (keyword_count as f32 / word_count as f32 * 100.0).min(1.0)
            },
            _ => 0.7, // Default score for other genres
        };

        // Overall quality score (weighted average)
        length_score * 0.3 + variety_score * 0.3 + genre_score * 0.4
    }

    /// Auto-save learning progress to disk
    async fn auto_save_learning(&self) -> Result<()> {
        let learning_system = self.advanced_learning.lock().unwrap();
        learning_system.save_to_disk()?;
        learning_system.auto_save_session(&self.session_id)?;
        Ok(())
    }

    /// Get enhanced writing recommendations
    pub fn get_writing_recommendations(&self, book: &Book) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Get recommendations from advanced learning system
        {
            let learning_system = self.advanced_learning.lock().unwrap();
            recommendations.extend(learning_system.generate_improvement_suggestions());
        }

        // Add book-specific recommendations
        if book.metadata.current_word_count < book.metadata.target_word_count.unwrap_or(50000) / 2 {
            recommendations.push("Consider expanding character development in upcoming chapters".to_string());
        }

        if book.chapters().len() > 5 {
            recommendations.push("Review previous chapters for consistency and flow".to_string());
        }

        recommendations
    }

    /// Shutdown and final save
    pub async fn shutdown(&self) -> Result<()> {
        println!("💾 Saving final learning state...");
        self.auto_save_learning().await?;
        println!("✅ Enhanced writer integration shutdown complete");
        Ok(())
    }
}

impl Default for EnhancedWriterIntegration {
    fn default() -> Self {
        Self::new().expect("Failed to initialize enhanced writer integration")
    }
}